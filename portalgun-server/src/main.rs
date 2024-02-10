/*
 * SPDX-FileCopyrightText: 2024 perillamint <perillamint@silicon.moe>
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use axum::body::Body;
use axum::extract::State;
use axum::routing::get;
use std::sync::Arc;
use url::Url;

use crate::grpc::health;
use axum::http::header::CONTENT_TYPE;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use clap::Parser;
use serde::Deserialize;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tower::steer::Steer;

mod grpc;

const GRPC_ROUTER_INDEX: usize = 0;
const AXUM_ROUTER_INDEX: usize = 1;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    println!("Hello, world!");

    let grpc_router = tonic::transport::Server::builder()
        .add_service(health::proto::health_server::HealthServer::new(
            health::HealthService {},
        ))
        .into_router();

    let axum_router = axum::Router::new()
        .route(
            "/api/v1/heartbeat",
            get(|| async { "Project Kaguya up and running." }),
        );

    let hybrid_svc = Steer::new(
        vec![grpc_router, axum_router],
        |req: &Request<Body>, _svcs: &[_]| match req
            .headers()
            .get(CONTENT_TYPE)
            .map(|hdr| hdr.as_bytes())
        {
            Some(b"application/grpc") => GRPC_ROUTER_INDEX,
            _ => AXUM_ROUTER_INDEX,
        },
    );

    let listen: std::net::SocketAddrV4 = "0.0.0.0:3000".parse().unwrap();

    axum::Server::bind(&listen.into())
        .serve(tower::make::Shared::new(hybrid_svc))
        .await
        .unwrap();

    Ok(())
}

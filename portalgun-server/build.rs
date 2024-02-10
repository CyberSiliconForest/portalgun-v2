// SPDX-FileCopyrightText: 2024 perillamint <perillamint@silicon.moe>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .compile(
            &["../proto/tunnel.proto"],
            &["../proto/"],
        )?;

    Ok(())
}

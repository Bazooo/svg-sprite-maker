import fs from 'fs'

const version = process.argv[2].replace(/^v/, '')

const cargoToml = fs.readFileSync('src-tauri/Cargo.toml', 'utf8')

const updatedCargoToml = cargoToml.replace(
    /version = "[^"]+"/,
    `version = "${version}"`)

fs.writeFileSync('src-tauri/Cargo.toml', updatedCargoToml)

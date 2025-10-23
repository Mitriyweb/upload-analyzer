#!/usr/bin/env node

const { execSync } = require('child_process');
const os = require('os');
const path = require('path');

// Add cargo bin to PATH
const homeDir = os.homedir();
const cargoPath = path.join(homeDir, '.cargo', 'bin');
process.env.PATH = `${cargoPath}:${process.env.PATH}`;

// Get target from command line args (bundler or web)
const target = process.argv[2] || 'bundler';

// Check if wasm-pack is available
try {
    execSync('wasm-pack --version', { stdio: 'ignore' });
} catch (error) {
    console.error('❌ wasm-pack not found!');
    console.error('   Install it with: cargo install wasm-pack');
    process.exit(1);
}

// Run wasm-pack build
try {
    execSync(`wasm-pack build --target ${target} --out-dir pkg`, { 
        stdio: 'inherit',
        cwd: path.join(__dirname, '..')
    });
} catch (error) {
    process.exit(error.status || 1);
}

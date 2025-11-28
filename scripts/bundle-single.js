const fs = require('fs');
const path = require('path');

// Read the WASM wrapper
const wrapperPath = path.join(__dirname, '../pkg/upload_analyzer_bg.js');
let wrapper = fs.readFileSync(wrapperPath, 'utf-8');

// Read the app code
const appPath = path.join(__dirname, '../dev/app.js');
let app = fs.readFileSync(appPath, 'utf-8');

// Remove the import line from app.js
app = app.replace(/^import.*from.*['\"].*['\"];?\\s*$/m, '');

// The wrapper already has init, analyze_file, and get_file_info defined
// We just need to make sure they're accessible in the bundle

// Create the bundle - keep the wrapper as-is, just remove the export statements
// and make the functions available globally
const bundle = `
${wrapper}

// Expose WASM functions globally for the app code
window.init = init;
window.analyze_file = analyze_file;
window.get_file_info = get_file_info;

// App code
${app}
`.trim();

// Write to public
const outputPath = path.join(__dirname, '../public/app.bundle.js');
fs.writeFileSync(outputPath, bundle);

console.log('✅ Created public/app.bundle.js');
console.log(`   Size: ${(bundle.length / 1024).toFixed(2)} KB`);

// Now minify it
const { execSync } = require('child_process');
try {
    execSync('npx terser public/app.bundle.js -o public/app.min.js -c -m --module', { stdio: 'inherit', cwd: path.join(__dirname, '..') });

    // Fix the wasm path in minified version
    let minified = fs.readFileSync(path.join(__dirname, '../public/app.min.js'), 'utf-8');
    minified = minified.replace(/['"]upload_analyzer_bg\.wasm['"]/g, '"./upload_analyzer_bg.wasm"');
    fs.writeFileSync(path.join(__dirname, '../public/app.min.js'), minified);

    // Remove the bundle source
    fs.unlinkSync(outputPath);

    console.log('✅ Created public/app.min.js (minified)');
} catch (error) {
    console.error('Error minifying:', error.message);
}

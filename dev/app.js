import init, { analyze_file, get_file_info } from '/pkg/upload_analyzer.js';

let wasmInitialized = false;
const status = document.getElementById('status');
const results = document.getElementById('results');
const output = document.getElementById('output');
const fileInput = document.getElementById('fileInput');
const uploadArea = document.getElementById('uploadArea');

async function initWasm() {
    try {
        status.innerHTML = '<div class="info">⏳ Loading analyzer...</div>';
        await init();
        wasmInitialized = true;
        status.innerHTML = '<div class="info">✅ Ready! Drop a file to analyze.</div>';
    } catch (error) {
        status.innerHTML = `<div class="error">❌ Failed to load analyzer: ${error}</div>`;
    }
}

fileInput.addEventListener('change', (e) => {
    if (e.target.files.length > 0) {
        handleFile(e.target.files[0]);
    }
});

uploadArea.addEventListener('dragover', (e) => {
    e.preventDefault();
    uploadArea.classList.add('dragover');
});

uploadArea.addEventListener('dragleave', () => {
    uploadArea.classList.remove('dragover');
});

uploadArea.addEventListener('drop', (e) => {
    e.preventDefault();
    uploadArea.classList.remove('dragover');
    if (e.dataTransfer.files.length > 0) {
        handleFile(e.dataTransfer.files[0]);
    }
});

async function handleFile(file) {
    if (!wasmInitialized) {
        status.innerHTML = '<div class="error">⚠️ Analyzer not ready yet. Please wait...</div>';
        return;
    }

    status.innerHTML = `<div class="info">🔍 Analyzing ${file.name}...</div>`;
    results.style.display = 'none';

    try {
        const buffer = await file.arrayBuffer();
        const data = new Uint8Array(buffer);

        const infoJson = get_file_info(data);
        const info = JSON.parse(infoJson);

        let html = '<h3>File Information</h3>';
        html += '<ul>';
        html += `<li><strong>Filename:</strong> ${file.name}</li>`;
        html += `<li><strong>Size:</strong> ${(file.size / 1024).toFixed(2)} KB (${file.size.toLocaleString()} bytes)</li>`;
        html += `<li><strong>Type:</strong> ${info.type || 'Unknown'}</li>`;
        html += '</ul>';
        
        html += '<h3>Basic Information</h3>';
        html += `<pre>${JSON.stringify(info, null, 2)}</pre>`;

        if (info.type && (info.type.includes('PE') || info.type.includes('MSI') || info.type.includes('DMG'))) {
            try {
                const analysisJson = analyze_file(data);
                const analysis = JSON.parse(analysisJson);
                
                if (analysis.error) {
                    html += `<div class="error">⚠️ Analysis error: ${analysis.error}</div>`;
                } else {
                    html += '<h3>Binary Metadata</h3>';
                    
                    if (Object.keys(analysis).length === 0) {
                        html += '<p>No metadata found.</p>';
                    } else {
                        html += '<h4>File Header</h4>';
                        html += '<ul>';
                        if (analysis.Format) html += `<li><strong>Format:</strong> ${analysis.Format}</li>`;
                        if (analysis.InstallerType) html += `<li><strong>⚙️ Installer Type:</strong> ${analysis.InstallerType}</li>`;
                        if (analysis.EmbeddedMSI) html += `<li><strong>📦 Contains Embedded MSI:</strong> Yes (at offset ${analysis.MSIOffset})</li>`;
                        if (analysis.Architecture) html += `<li><strong>Architecture:</strong> ${analysis.Architecture}</li>`;
                        if (analysis.Machine) html += `<li><strong>Machine Type:</strong> ${analysis.Machine}</li>`;
                        if (analysis.NumberOfSections) html += `<li><strong>Number of Sections:</strong> ${analysis.NumberOfSections}</li>`;
                        if (analysis.Characteristics) html += `<li><strong>Characteristics:</strong> ${analysis.Characteristics}</li>`;
                        if (analysis.PointerToSymbolTable) html += `<li><strong>Symbol Table Pointer:</strong> ${analysis.PointerToSymbolTable}</li>`;
                        if (analysis.NumberOfSymbols) html += `<li><strong>Number of Symbols:</strong> ${analysis.NumberOfSymbols}</li>`;
                        if (analysis.Timestamp) {
                            const date = new Date(parseInt(analysis.Timestamp) * 1000);
                            html += `<li><strong>Compile Date:</strong> ${date.toUTCString()}</li>`;
                        }
                        html += '</ul>';
                        
                        html += '<h4>Optional Header</h4>';
                        html += '<ul>';
                        if (analysis.EntryPoint) html += `<li><strong>Entry Point:</strong> ${analysis.EntryPoint}</li>`;
                        if (analysis.ImageBase) html += `<li><strong>Image Base:</strong> ${analysis.ImageBase}</li>`;
                        if (analysis.SizeOfImage) html += `<li><strong>Size of Image:</strong> ${analysis.SizeOfImage} bytes</li>`;
                        if (analysis.Subsystem) html += `<li><strong>Subsystem:</strong> ${analysis.Subsystem}</li>`;
                        if (analysis.DllCharacteristics) html += `<li><strong>DLL Characteristics:</strong> ${analysis.DllCharacteristics}</li>`;
                        html += '</ul>';
                        
                        if (analysis.Format === 'MSI') {
                            html += '<h4>MSI Package Information</h4>';
                            html += '<ul>';
                            if (analysis.ProductCode) html += `<li><strong>Product Code:</strong> ${analysis.ProductCode}</li>`;
                            if (analysis.UpgradeCode) html += `<li><strong>Upgrade Code:</strong> ${analysis.UpgradeCode}</li>`;
                            if (analysis.InstallerFramework) html += `<li><strong>⚙️ Created With:</strong> ${analysis.InstallerFramework}</li>`;
                            html += '</ul>';
                        }
                        
                        if (analysis.Format === 'DMG') {
                            html += '<h4>DMG Disk Image Information</h4>';
                            html += '<ul>';
                            if (analysis.ImageType) html += `<li><strong>Image Type:</strong> ${analysis.ImageType}</li>`;
                            if (analysis.Compression) html += `<li><strong>Compression:</strong> ${analysis.Compression}</li>`;
                            if (analysis.HasKolySignature) html += `<li><strong>UDIF Signature:</strong> ${analysis.HasKolySignature === 'true' ? '✓ Present' : '✗ Missing'}</li>`;
                            if (analysis.DMGVersion) html += `<li><strong>DMG Version:</strong> ${analysis.DMGVersion}</li>`;
                            html += '</ul>';
                            
                            html += '<h4>Application Bundle Information</h4>';
                            html += '<ul>';
                            if (analysis.BundleIdentifier) html += `<li><strong>Bundle ID:</strong> ${analysis.BundleIdentifier}</li>`;
                            if (analysis.ApplicationBundle) html += `<li><strong>Application:</strong> ${analysis.ApplicationBundle}</li>`;
                            if (analysis.ExecutableName) html += `<li><strong>Executable:</strong> ${analysis.ExecutableName}</li>`;
                            if (analysis.PackageType) html += `<li><strong>Package Type:</strong> ${analysis.PackageType}</li>`;
                            if (analysis.ApplicationCategory) html += `<li><strong>📁 Category:</strong> ${analysis.ApplicationCategory}</li>`;
                            if (analysis.MinimumSystemVersion) html += `<li><strong>Min macOS:</strong> ${analysis.MinimumSystemVersion}</li>`;
                            if (analysis.IconFile) html += `<li><strong>Icon:</strong> ${analysis.IconFile}</li>`;
                            if (analysis.PrincipalClass) html += `<li><strong>Principal Class:</strong> ${analysis.PrincipalClass}</li>`;
                            html += '</ul>';
                        }
                        
                        html += '<h4>Version Information</h4>';
                        html += '<ul>';
                        if (analysis.FileVersionNumber) html += `<li><strong>File Version (Binary):</strong> ${analysis.FileVersionNumber}</li>`;
                        if (analysis.ProductVersionNumber) html += `<li><strong>Product Version (Binary):</strong> ${analysis.ProductVersionNumber}</li>`;
                        if (analysis.FileVersion) html += `<li><strong>File Version (String):</strong> ${analysis.FileVersion}</li>`;
                        if (analysis.ProductVersion) html += `<li><strong>Product Version (String):</strong> ${analysis.ProductVersion}</li>`;
                        if (analysis.SignedBy) html += `<li><strong>🔐 Digitally Signed By:</strong> ${analysis.SignedBy}</li>`;
                        if (analysis.CompanyName) {
                            const source = analysis.NoStringsFound ? ' (from digital signature)' : '';
                            html += `<li><strong>Company Name:</strong> ${analysis.CompanyName}${source}</li>`;
                        }
                        if (analysis.Manufacturer) html += `<li><strong>Manufacturer:</strong> ${analysis.Manufacturer}</li>`;
                        if (analysis.ProductName) html += `<li><strong>Product Name:</strong> ${analysis.ProductName}</li>`;
                        if (analysis.FileDescription) html += `<li><strong>File Description:</strong> ${analysis.FileDescription}</li>`;
                        if (analysis.InternalName) html += `<li><strong>Internal Name:</strong> ${analysis.InternalName}</li>`;
                        if (analysis.OriginalFilename) html += `<li><strong>Original Filename:</strong> ${analysis.OriginalFilename}</li>`;
                        if (analysis.LegalCopyright) html += `<li><strong>Legal Copyright:</strong> ${analysis.LegalCopyright}</li>`;
                        if (analysis.LegalTrademarks) html += `<li><strong>Legal Trademarks:</strong> ${analysis.LegalTrademarks}</li>`;
                        if (analysis.Comments) html += `<li><strong>Comments:</strong> ${analysis.Comments}</li>`;
                        if (analysis.PrivateBuild) html += `<li><strong>Private Build:</strong> ${analysis.PrivateBuild}</li>`;
                        if (analysis.SpecialBuild) html += `<li><strong>Special Build:</strong> ${analysis.SpecialBuild}</li>`;
                        if (analysis.FileType) html += `<li><strong>File Type:</strong> ${analysis.FileType}</li>`;
                        if (analysis.FileOS) html += `<li><strong>File OS:</strong> ${analysis.FileOS}</li>`;
                        if (analysis.FileFlags) html += `<li><strong>File Flags:</strong> ${analysis.FileFlags}</li>`;
                        html += '</ul>';
                        
                        html += '<h3>Complete JSON Data</h3>';
                        html += `<pre>${JSON.stringify(analysis, null, 2)}</pre>`;
                    }
                }
            } catch (error) {
                html += `<div class="error">⚠️ Detailed analysis failed: ${error}</div>`;
            }
        }

        output.innerHTML = html;
        results.style.display = 'block';
        status.innerHTML = '<div class="info">✅ Analysis complete!</div>';
    } catch (error) {
        status.innerHTML = `<div class="error">❌ Error analyzing file: ${error}</div>`;
    }
}

initWasm();

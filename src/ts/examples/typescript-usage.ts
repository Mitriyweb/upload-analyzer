/**
 * Example usage of upload-analyzer TypeScript types
 */

import type { 
  UploadAnalyzerWASM, 
  FileInfo, 
  FileAnalysis,
  PEAnalysis,
  MSIAnalysis
} from 'upload-analyzer/types';

import { 
  parseFileInfo,
  parseAnalysis,
  isPEAnalysis,
  isMSIAnalysis,
  isAnalysisError
} from 'upload-analyzer/helpers';

// Import the WASM module (adjust path as needed)
declare const wasm: UploadAnalyzerWASM;

// ========== Example 1: Get Basic File Info ==========

async function getBasicInfo(file: File): Promise<FileInfo> {
  const arrayBuffer = await file.arrayBuffer();
  const data = new Uint8Array(arrayBuffer);
  
  const jsonResult = wasm.get_file_info(data);
  const fileInfo = JSON.parse(jsonResult) as FileInfo;
  
  console.log(`File type: ${fileInfo.type}`);
  console.log(`File size: ${fileInfo.size} bytes`);
  
  return fileInfo;
}

// ========== Example 2: Analyze PE File ==========

async function analyzePEFile(file: File): Promise<void> {
  const arrayBuffer = await file.arrayBuffer();
  const data = new Uint8Array(arrayBuffer);
  
  const jsonResult = wasm.analyze_pe_file(data);
  const analysis = JSON.parse(jsonResult) as FileAnalysis;
  
  // Check for errors
  if (isAnalysisError(analysis)) {
    console.error('Analysis failed:', analysis.error);
    return;
  }
  
  // Type-safe handling based on format
  if (isPEAnalysis(analysis)) {
    console.log('PE File Analysis:');
    console.log('- Architecture:', analysis.Architecture);
    console.log('- Product Name:', analysis.ProductName);
    console.log('- Company Name:', analysis.CompanyName);
    console.log('- File Version:', analysis.FileVersionNumber);
    console.log('- Product Version:', analysis.ProductVersionNumber);
    
    if (analysis.SignedBy) {
      console.log('- Digitally Signed By:', analysis.SignedBy);
    }
    
    if (analysis.InstallerType) {
      console.log('- Installer Type:', analysis.InstallerType);
    }
    
    if (analysis.EmbeddedMSI === 'true') {
      console.log('- Contains Embedded MSI at offset:', analysis.MSIOffset);
    }
  } else if (isMSIAnalysis(analysis)) {
    console.log('MSI Package Analysis:');
    console.log('- Product Name:', analysis.ProductName);
    console.log('- Manufacturer:', analysis.Manufacturer);
    console.log('- Product Version:', analysis.ProductVersion);
    console.log('- Product Code:', analysis.ProductCode);
    console.log('- Upgrade Code:', analysis.UpgradeCode);
    
    if (analysis.InstallerFramework) {
      console.log('- Created With:', analysis.InstallerFramework);
    }
  }
}

// ========== Example 3: Extract Specific Fields ==========

function extractCompanyInfo(analysis: FileAnalysis): {
  company?: string;
  product?: string;
  version?: string;
} {
  if (isAnalysisError(analysis)) {
    return {};
  }
  
  if (isPEAnalysis(analysis)) {
    return {
      company: analysis.CompanyName || analysis.Publisher || analysis.SignedBy,
      product: analysis.ProductName || analysis.FileDescription,
      version: analysis.ProductVersion || analysis.FileVersion || analysis.ProductVersionNumber
    };
  }
  
  if (isMSIAnalysis(analysis)) {
    return {
      company: analysis.Manufacturer || analysis.CompanyName,
      product: analysis.ProductName || analysis.Product,
      version: analysis.ProductVersion || analysis.Version
    };
  }
  
  return {};
}

// ========== Example 4: Detect File Type and Analyze ==========

async function detectAndAnalyze(file: File): Promise<void> {
  const arrayBuffer = await file.arrayBuffer();
  const data = new Uint8Array(arrayBuffer);
  
  // First get basic file info
  const infoJson = wasm.get_file_info(data);
  const info = JSON.parse(infoJson) as FileInfo;
  
  console.log(`Detected file type: ${info.type}`);
  
  // Then perform detailed analysis
  if (info.type.includes('PE') || info.type.includes('MSI')) {
    const analysisJson = wasm.analyze_pe_file(data);
    const analysis = JSON.parse(analysisJson) as FileAnalysis;
    
    if (!isAnalysisError(analysis)) {
      const companyInfo = extractCompanyInfo(analysis);
      console.log('Company Info:', companyInfo);
    }
  }
}

// ========== Example 5: Type-safe Field Access ==========

function displayVersionInfo(analysis: PEAnalysis): string {
  const lines: string[] = [];
  
  if (analysis.FileVersionNumber) {
    lines.push(`File Version: ${analysis.FileVersionNumber}`);
  }
  
  if (analysis.ProductVersionNumber) {
    lines.push(`Product Version: ${analysis.ProductVersionNumber}`);
  }
  
  if (analysis.CompanyName) {
    const source = analysis.NoStringsFound === 'true' ? ' (from digital signature)' : '';
    lines.push(`Company: ${analysis.CompanyName}${source}`);
  }
  
  if (analysis.ProductName) {
    lines.push(`Product: ${analysis.ProductName}`);
  }
  
  return lines.join('\n');
}

// ========== Example 6: Handle File Upload in React/Vue/etc ==========

function handleFileUpload(event: Event): void {
  const input = event.target as HTMLInputElement;
  const file = input.files?.[0];
  
  if (!file) return;
  
  const reader = new FileReader();
  
  reader.onload = (e) => {
    const arrayBuffer = e.target?.result as ArrayBuffer;
    const data = new Uint8Array(arrayBuffer);
    
    try {
      const analysisJson = wasm.analyze_pe_file(data);
      const analysis = JSON.parse(analysisJson) as FileAnalysis;
      
      if (isAnalysisError(analysis)) {
        console.error('Analysis error:', analysis.error);
        return;
      }
      
      if (isPEAnalysis(analysis)) {
        const versionInfo = displayVersionInfo(analysis);
        console.log(versionInfo);
      }
    } catch (error) {
      console.error('Failed to analyze file:', error);
    }
  };
  
  reader.readAsArrayBuffer(file);
}

export {
  getBasicInfo,
  analyzePEFile,
  extractCompanyInfo,
  detectAndAnalyze,
  displayVersionInfo,
  handleFileUpload
};

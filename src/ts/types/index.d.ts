/**
 * TypeScript type definitions for upload-analyzer WASM module
 */

// ========== Basic File Info Types ==========

export interface FileInfo {
  type: string;
  size: string;
}

// ========== PE File Analysis Types ==========

export interface PEAnalysis {
  // Basic Format
  Format: "PE";
  Architecture: "x86" | "x64";

  // File Header
  Machine?: string;
  NumberOfSections?: string;
  SizeOfOptionalHeader?: string;
  Characteristics?: string;
  PointerToSymbolTable?: string;
  NumberOfSymbols?: string;
  Timestamp?: string;

  // Optional Header
  EntryPoint?: string;
  ImageBase?: string;
  SizeOfImage?: string;
  Subsystem?: string;
  DllCharacteristics?: string;

  // Version Information
  HasVersionInfo?: "true" | "false";
  HasResources?: "true" | "false";
  FileVersionNumber?: string;
  ProductVersionNumber?: string;
  FileVersion?: string;
  ProductVersion?: string;
  FileFlags?: string;
  FileOS?: string;
  FileType?: string;

  // String Version Info
  CompanyName?: string;
  ProductName?: string;
  FileDescription?: string;
  InternalName?: string;
  OriginalFilename?: string;
  LegalCopyright?: string;
  LegalTrademarks?: string;
  Comments?: string;
  PrivateBuild?: string;
  SpecialBuild?: string;

  // Aliases
  ProgramName?: string;
  Vendor?: string;
  Publisher?: string;
  Version?: string;

  // Digital Signature
  SignedBy?: string;

  // Installer Detection
  InstallerType?: "Inno Setup" | "NSIS (Nullsoft)" | "InstallShield" | "WiX Toolset" | "Wise Installer" | "Setup Factory" | "Smart Install Maker";
  EmbeddedMSI?: "true" | "false";
  MSIOffset?: string;

  // Translation/Language
  TranslationCount?: string;
  Language?: string;
  StringsCount?: string;
  NoStringsFound?: "true" | "false";

  // Debug fields (when present)
  Translation_0?: string;
  StringsInTranslation_0?: string;
  TotalCallbackCalls?: string;
  [key: `Debug_${number}_${string}`]: string | undefined;
  [key: `Translation_${number}`]: string | undefined;
  [key: `StringsInTranslation_${number}`]: string | undefined;

  // Error states
  VersionInfoError?: string;
  ResourcesError?: string;
  CompoundFileError?: string;
}

// ========== MSI File Analysis Types ==========

export interface MSIAnalysis {
  // Basic Format
  Format: "MSI";
  Architecture: "Windows Installer Package";

  // MSI Package Information
  ProductCode?: string;
  UpgradeCode?: string;
  ProductVersion?: string;
  Version?: string;

  // Product Information
  ProductName?: string;
  Product?: string;
  Manufacturer?: string;
  CompanyName?: string;
  Publisher?: string;
  Comments?: string;

  // Installer Framework
  InstallerFramework?: "WiX Toolset" | "InstallShield" | "Advanced Installer";

  // Compound File
  HasCompoundFile?: "true" | "false";
  HasSummaryInfo?: "true" | "false";
  CompoundFileError?: string;
}

// ========== DMG File Analysis Types ==========

export interface DMGAnalysis {
  // Basic Format
  Format: "DMG";
  Architecture: "macOS Disk Image";

  // DMG Information
  ImageType?: string;
  Compression?: string;
  HasKolySignature?: "true" | "false";
  KolyOffset?: string;
  DMGVersion?: string;

  // Product Information (matching PE fields)
  ProductName?: string;
  ProgramName?: string;
  DisplayName?: string;
  FileDescription?: string;

  // Version Information (matching PE fields)
  ProductVersion?: string;
  FileVersion?: string;
  FileVersionNumber?: string;
  ProductVersionNumber?: string;

  // Company Information (matching PE fields)
  CompanyName?: string;
  Manufacturer?: string;
  Vendor?: string;
  Publisher?: string;

  // Legal Information
  LegalCopyright?: string;

  // macOS Specific
  BundleIdentifier?: string;
  ApplicationBundle?: string;
  ApplicationCategory?: string;
  PrincipalClass?: string;
  ExecutableName?: string;
  PackageType?: string;
  IconFile?: string;
  MinimumSystemVersion?: string;
}

// ========== DEB File Analysis Types ==========

export interface DEBAnalysis {
  // Basic Format
  Format: "DEB";
  Architecture?: string;

  // DEB Package Information
  Package?: string;
  Version?: string;
  Maintainer?: string;
  Description?: string;
  Section?: string;
  Priority?: string;
  Depends?: string;
  Homepage?: string;

  // Compatibility Aliases (matching PE/MSI fields)
  ProductName?: string;
  ProductVersion?: string;
  CompanyName?: string;
}

// ========== Error Response ==========

export interface AnalysisError {
  error: string;
}

// ========== Union Types ==========

export type FileAnalysis =
  | PEAnalysis
  | MSIAnalysis
  | DMGAnalysis
  | DEBAnalysis
  | AnalysisError;

// ========== WASM Module Interface ==========

export interface UploadAnalyzerWASM {
  /**
   * Initialize panic hook for better error messages
   */
  init_panic_hook(): void;

  /**
   * Get basic file type and size information
   * @param data - File data as Uint8Array
   * @returns JSON string containing file type and size
   */
  get_file_info(data: Uint8Array): string;

  /**
   * Analyze PE/MSI file and extract metadata
   * @param data - File data as Uint8Array
   * @returns JSON string containing detailed metadata
   */
  analyze_pe_file(data: Uint8Array): string;

  /**
   * Analyze any supported file format (alias for analyze_pe_file)
   * @param data - File data as Uint8Array
   * @returns JSON string containing detailed metadata
   */
  analyze_file(data: Uint8Array): string;
}

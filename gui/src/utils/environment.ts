import * as path from 'path';

export const MACOS = process.platform === 'darwin';
export const WIN = process.platform === 'win32';

// project dirs
const PROJECT_ROOT = path.resolve(__dirname, '..', '..');
// const SOURCE_DIR = path.join(PROJECT_ROOT, '..');

// xi-core
export const XI_CORE_DIR = path.join(PROJECT_ROOT, '../target/debug/');
export const XI_CORE_BIN = path.join(XI_CORE_DIR, WIN ? 'stadal.exe' : 'stadal');

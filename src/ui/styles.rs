pub const STYLESHEET: &str = r#"
* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", "PingFang SC", "Microsoft YaHei", sans-serif;
    background: #1a1a2e;
    color: #e0e0e0;
    overflow: hidden;
    height: 100vh;
}

.app {
    display: flex;
    flex-direction: column;
    height: 100vh;
}

/* Top Panel */
.top-panel {
    padding: 12px 20px;
    background: #16213e;
    border-bottom: 1px solid #2a2a4a;
    flex-shrink: 0;
}

.header-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 8px;
}

.title {
    font-size: 20px;
    font-weight: 700;
    color: #f0f0f0;
}

.scan-info {
    font-size: 13px;
    color: #888;
}

.controls-row {
    display: flex;
    align-items: center;
    gap: 10px;
}

.scan-root {
    font-family: monospace;
    font-size: 13px;
    color: #64b5f6;
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.btn {
    padding: 6px 14px;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-size: 13px;
    font-weight: 500;
    transition: all 0.15s;
    white-space: nowrap;
}

.btn:hover {
    filter: brightness(1.15);
}

.btn:active {
    transform: scale(0.97);
}

.btn-primary {
    background: #3b82f6;
    color: white;
}

.btn-secondary {
    background: #374151;
    color: #d1d5db;
}

.btn-danger {
    background: #dc2626;
    color: white;
}

.btn-danger:hover {
    background: #ef4444;
}

.btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
    filter: none;
    transform: none;
}

.scan-root-input {
    flex: 1;
    padding: 5px 10px;
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 6px;
    color: #64b5f6;
    font-family: monospace;
    font-size: 13px;
    outline: none;
}

.scan-root-input:focus {
    border-color: #3b82f6;
}

/* Toolbar */
.toolbar {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 20px;
    background: #1e293b;
    border-bottom: 1px solid #2a2a4a;
    flex-shrink: 0;
}

.search-input {
    padding: 5px 10px;
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 6px;
    color: #e0e0e0;
    font-size: 13px;
    width: 260px;
    outline: none;
}

.search-input:focus {
    border-color: #3b82f6;
}

.sort-label {
    font-size: 13px;
    color: #888;
    margin-left: 6px;
}

.sort-btn {
    padding: 4px 10px;
    border: 1px solid #334155;
    border-radius: 4px;
    background: transparent;
    color: #9ca3af;
    cursor: pointer;
    font-size: 12px;
    transition: all 0.15s;
}

.sort-btn:hover {
    border-color: #3b82f6;
    color: #e0e0e0;
}

.sort-btn.active {
    background: #3b82f6;
    border-color: #3b82f6;
    color: white;
}

/* Project List */
.project-list {
    flex: 1;
    overflow-y: auto;
    padding: 10px 20px;
}

.empty-state {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: #666;
    font-size: 15px;
}

.project-card {
    display: flex;
    flex-direction: column;
    padding: 10px 14px;
    margin-bottom: 6px;
    background: #1e293b;
    border-radius: 8px;
    border: 1px solid transparent;
    transition: all 0.15s;
}

.project-card:hover {
    border-color: #334155;
    background: #243047;
}

.project-card.selected {
    background: #1e3a5f;
    border-color: #3b82f6;
}

.project-main-row {
    display: flex;
    align-items: flex-start;
    gap: 10px;
    cursor: pointer;
}

.project-card input[type="checkbox"] {
    margin-top: 3px;
    accent-color: #3b82f6;
    width: 16px;
    height: 16px;
    cursor: pointer;
}

.project-info {
    flex: 1;
    min-width: 0;
}

.project-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 2px;
}

.project-name {
    font-size: 14px;
    font-weight: 600;
    color: #f0f0f0;
}

.project-size {
    font-size: 14px;
    font-weight: 700;
    white-space: nowrap;
}

.project-path {
    font-family: monospace;
    font-size: 11px;
    color: #6b7280;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    margin-bottom: 4px;
}

.build-targets {
    display: flex;
    align-items: center;
    gap: 6px;
    flex-wrap: wrap;
    margin-top: 6px;
    padding-top: 6px;
    border-top: 1px solid #2a2a4a;
    margin-left: 26px;
}

.targets-label {
    font-size: 11px;
    color: #9ca3af;
}

.target-item {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    padding: 2px 8px;
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.15s;
    font-size: 12px;
}

.target-item:hover {
    border-color: #3b82f6;
    background: #1e293b;
}

.target-item.selected {
    background: #1e3a5f;
    border-color: #3b82f6;
}

.target-item input[type="checkbox"] {
    width: 12px;
    height: 12px;
    accent-color: #3b82f6;
    cursor: pointer;
}

.target-name {
    color: #bfdbfe;
    font-size: 11px;
}

.target-size {
    color: #9ca3af;
    font-size: 10px;
    margin-left: 2px;
}

/* Size Colors */
.size-green { color: #4ade80; }
.size-yellow { color: #facc15; }
.size-orange { color: #fb923c; }
.size-red { color: #f87171; }

/* Bottom Panel */
.bottom-panel {
    padding: 10px 20px;
    background: #16213e;
    border-top: 1px solid #2a2a4a;
    flex-shrink: 0;
}

.status-msg {
    font-size: 13px;
    color: #4ade80;
    margin-bottom: 6px;
}

.bottom-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
}

.stats {
    display: flex;
    align-items: center;
    gap: 16px;
}

.total-size {
    font-size: 15px;
    font-weight: 700;
    color: #f0f0f0;
}

.selected-info {
    font-size: 13px;
    color: #facc15;
}

.actions {
    display: flex;
    align-items: center;
    gap: 8px;
}

.confirm-text {
    font-size: 13px;
    color: #f87171;
    font-weight: 600;
}

.spinner {
    display: inline-block;
    width: 14px;
    height: 14px;
    border: 2px solid #3b82f6;
    border-top-color: transparent;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
    margin-right: 6px;
    vertical-align: middle;
}

@keyframes spin {
    to { transform: rotate(360deg); }
}

/* Loading Overlay */
.loading-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: rgba(0, 0, 0, 0.6);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    backdrop-filter: blur(2px);
}

.loading-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 16px;
    padding: 32px 48px;
    background: #1e293b;
    border-radius: 12px;
    border: 1px solid #334155;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
}

.loading-spinner {
    width: 40px;
    height: 40px;
    border: 4px solid #334155;
    border-top-color: #3b82f6;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
}

.loading-text {
    font-size: 14px;
    color: #94a3b8;
}

/* Toast Notification */
.toast {
    position: fixed;
    top: 20px;
    left: 50%;
    transform: translateX(-50%);
    padding: 12px 24px;
    border-radius: 8px;
    font-size: 14px;
    font-weight: 500;
    z-index: 2000;
    cursor: pointer;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.3);
    animation: toast-in 0.3s ease-out;
    max-width: 80%;
    text-align: center;
}

.toast-success {
    background: #065f46;
    color: #a7f3d0;
    border: 1px solid #10b981;
}

.toast-error {
    background: #7f1d1d;
    color: #fca5a5;
    border: 1px solid #ef4444;
}

@keyframes toast-in {
    from {
        opacity: 0;
        transform: translateX(-50%) translateY(-20px);
    }
    to {
        opacity: 1;
        transform: translateX(-50%) translateY(0);
    }
}

/* Scrollbar */
.project-list::-webkit-scrollbar {
    width: 8px;
}
.project-list::-webkit-scrollbar-track {
    background: #1a1a2e;
}
.project-list::-webkit-scrollbar-thumb {
    background: #374151;
    border-radius: 4px;
}
.project-list::-webkit-scrollbar-thumb:hover {
    background: #4b5563;
}
"#;

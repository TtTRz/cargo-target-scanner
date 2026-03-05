pub const STYLESHEET: &str = r#"
* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

body {
    font-family: -apple-system, BlinkMacSystemFont, "Helvetica Neue", "PingFang SC", "Microsoft YaHei", sans-serif;
    background: #EDEDED;
    color: rgba(0, 0, 0, 0.9);
    overflow: hidden;
    height: 100vh;
    font-size: 14px;
    -webkit-font-smoothing: antialiased;
}

.app {
    display: flex;
    flex-direction: column;
    height: 100vh;
}

/* ========== Top Panel (WeUI Navigation Bar) ========== */
.top-panel {
    padding: 12px 16px;
    background: #EDEDED;
    border-bottom: 1px solid rgba(0, 0, 0, 0.05);
    flex-shrink: 0;
}

.header-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 12px;
}

.header-right {
    display: flex;
    align-items: center;
    gap: 12px;
}

.btn-lang {
    padding: 4px 12px;
    background: rgba(0, 0, 0, 0.05);
    border: 1px solid rgba(0, 0, 0, 0.1);
    border-radius: 4px;
    font-size: 12px;
    color: rgba(0, 0, 0, 0.6);
    cursor: pointer;
    transition: all 0.15s;
}

.btn-lang:hover {
    border-color: #07C160;
    color: #07C160;
}

.title {
    font-size: 17px;
    font-weight: 600;
    color: rgba(0, 0, 0, 0.9);
}

.scan-info {
    font-size: 12px;
    color: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
}

.controls-row {
    display: flex;
    align-items: center;
    gap: 8px;
}

.controls-row > span {
    font-size: 14px;
    color: rgba(0, 0, 0, 0.5);
    white-space: nowrap;
}

/* ========== WeUI Buttons ========== */
.btn {
    padding: 6px 16px;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-size: 14px;
    font-weight: 500;
    transition: opacity 0.15s;
    white-space: nowrap;
    line-height: 1.4;
}

.btn:hover {
    opacity: 0.85;
}

.btn:active {
    opacity: 0.7;
}

.btn-primary {
    background: #07C160;
    color: #FFFFFF;
}

.btn-secondary {
    background: rgba(0, 0, 0, 0.05);
    color: rgba(0, 0, 0, 0.9);
    border: 1px solid rgba(0, 0, 0, 0.1);
}

.btn-danger {
    background: #FA5151;
    color: #FFFFFF;
}

.btn-danger:hover {
    opacity: 0.85;
}

.btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
}

/* ========== WeUI Input ========== */
.scan-root-input {
    flex: 1;
    padding: 6px 12px;
    background: #FFFFFF;
    border: 1px solid rgba(0, 0, 0, 0.1);
    border-radius: 6px;
    color: rgba(0, 0, 0, 0.9);
    font-family: -apple-system, "SF Mono", "Menlo", monospace;
    font-size: 13px;
    outline: none;
    transition: border-color 0.2s;
}

.scan-root-input:focus {
    border-color: #07C160;
}

/* ========== Toolbar (WeUI SearchBar style) ========== */
.toolbar {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 16px;
    background: #EDEDED;
    flex-shrink: 0;
}

.toolbar > span:first-child {
    color: rgba(0, 0, 0, 0.4);
    font-size: 14px;
}

.search-input {
    padding: 7px 12px;
    background: #FFFFFF;
    border: none;
    border-radius: 6px;
    color: rgba(0, 0, 0, 0.9);
    font-size: 14px;
    width: 240px;
    outline: none;
}

.search-input::placeholder {
    color: rgba(0, 0, 0, 0.3);
}

.sort-label {
    font-size: 13px;
    color: rgba(0, 0, 0, 0.5);
    margin-left: 4px;
}

.sort-btn {
    padding: 4px 12px;
    border: 1px solid rgba(0, 0, 0, 0.1);
    border-radius: 4px;
    background: #FFFFFF;
    color: rgba(0, 0, 0, 0.5);
    cursor: pointer;
    font-size: 13px;
    transition: all 0.15s;
}

.sort-btn:hover {
    border-color: #07C160;
    color: #07C160;
}

.sort-btn.active {
    background: #07C160;
    border-color: #07C160;
    color: #FFFFFF;
}

/* ========== Project List (WeUI Cells) ========== */
.project-list {
    flex: 1;
    overflow-y: auto;
    padding: 0;
    background: #EDEDED;
}

.empty-state {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: rgba(0, 0, 0, 0.3);
    font-size: 15px;
}

/* ========== Project Card (WeUI Cell) ========== */
.project-card {
    display: flex;
    flex-direction: column;
    padding: 12px 16px;
    background: #FFFFFF;
    border-bottom: 1px solid rgba(0, 0, 0, 0.05);
    transition: background 0.15s;
}

.project-card:first-child {
    border-top: 1px solid rgba(0, 0, 0, 0.05);
}

.project-card:hover {
    background: #F7F7F7;
}

.project-card.selected {
    background: rgba(7, 193, 96, 0.06);
    border-left: 3px solid #07C160;
}

.project-main-row {
    display: flex;
    align-items: flex-start;
    gap: 12px;
    cursor: pointer;
}

.project-card input[type="checkbox"] {
    margin-top: 2px;
    accent-color: #07C160;
    width: 18px;
    height: 18px;
    cursor: pointer;
    flex-shrink: 0;
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
    font-size: 15px;
    font-weight: 500;
    color: rgba(0, 0, 0, 0.9);
}

.project-size {
    font-size: 14px;
    font-weight: 600;
    white-space: nowrap;
}

.project-path {
    font-family: -apple-system, "SF Mono", "Menlo", monospace;
    font-size: 12px;
    color: rgba(0, 0, 0, 0.4);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    margin-bottom: 2px;
}

/* ========== Build Targets ========== */
.build-targets {
    display: flex;
    align-items: center;
    gap: 6px;
    flex-wrap: wrap;
    margin-top: 8px;
    padding-top: 8px;
    border-top: 1px solid rgba(0, 0, 0, 0.05);
    margin-left: 30px;
}

.targets-label {
    font-size: 12px;
    color: rgba(0, 0, 0, 0.4);
}

.target-item {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    padding: 3px 10px;
    background: #F2F2F2;
    border: 1px solid rgba(0, 0, 0, 0.06);
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.15s;
    font-size: 12px;
}

.target-item:hover {
    border-color: #07C160;
    background: rgba(7, 193, 96, 0.06);
}

.target-item.selected {
    background: rgba(7, 193, 96, 0.1);
    border-color: #07C160;
}

.target-item input[type="checkbox"] {
    width: 13px;
    height: 13px;
    accent-color: #07C160;
    cursor: pointer;
}

.target-name {
    color: rgba(0, 0, 0, 0.7);
    font-size: 12px;
}

.target-size {
    color: rgba(0, 0, 0, 0.4);
    font-size: 11px;
    margin-left: 2px;
}

/* ========== Size Colors (WeUI tinted) ========== */
.size-green { color: #07C160; }
.size-yellow { color: #EDA20C; }
.size-orange { color: #FA9D3B; }
.size-red { color: #FA5151; }

/* ========== Bottom Panel (WeUI Tabbar style) ========== */
.bottom-panel {
    padding: 10px 16px;
    background: #FFFFFF;
    border-top: 1px solid rgba(0, 0, 0, 0.08);
    flex-shrink: 0;
}

.status-msg {
    font-size: 13px;
    color: #07C160;
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
    font-weight: 600;
    color: rgba(0, 0, 0, 0.9);
}

.selected-info {
    font-size: 13px;
    color: #FA9D3B;
}

.actions {
    display: flex;
    align-items: center;
    gap: 8px;
}

.confirm-text {
    font-size: 13px;
    color: #FA5151;
    font-weight: 500;
}

/* ========== Spinner (WeUI Loading) ========== */
.spinner {
    display: inline-block;
    width: 14px;
    height: 14px;
    border: 2px solid #07C160;
    border-top-color: transparent;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
    margin-right: 6px;
    vertical-align: middle;
}

@keyframes spin {
    to { transform: rotate(360deg); }
}

/* ========== Loading Overlay (WeUI Loading Toast) ========== */
.loading-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
}

.loading-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    padding: 28px 36px;
    background: rgba(0, 0, 0, 0.8);
    border-radius: 12px;
    min-width: 120px;
}

.loading-spinner {
    width: 36px;
    height: 36px;
    border: 3px solid rgba(255, 255, 255, 0.2);
    border-top-color: #FFFFFF;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
}

.loading-text {
    font-size: 14px;
    color: #FFFFFF;
    white-space: nowrap;
}

/* ========== Toast (WeUI Toast) ========== */
.toast {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    padding: 16px 24px;
    border-radius: 8px;
    font-size: 14px;
    font-weight: 400;
    z-index: 2000;
    cursor: pointer;
    box-shadow: 0 2px 12px rgba(0, 0, 0, 0.15);
    animation: weui-fade-in 0.2s ease-out;
    max-width: 80%;
    text-align: center;
    line-height: 1.6;
}

.toast-success {
    background: rgba(0, 0, 0, 0.8);
    color: #FFFFFF;
}

.toast-error {
    background: rgba(0, 0, 0, 0.8);
    color: #FFFFFF;
}

@keyframes weui-fade-in {
    from {
        opacity: 0;
        transform: translate(-50%, -50%) scale(0.9);
    }
    to {
        opacity: 1;
        transform: translate(-50%, -50%) scale(1);
    }
}

/* ========== Scrollbar (subtle) ========== */
.project-list::-webkit-scrollbar {
    width: 4px;
}
.project-list::-webkit-scrollbar-track {
    background: transparent;
}
.project-list::-webkit-scrollbar-thumb {
    background: rgba(0, 0, 0, 0.15);
    border-radius: 2px;
}
.project-list::-webkit-scrollbar-thumb:hover {
    background: rgba(0, 0, 0, 0.25);
}
"#;

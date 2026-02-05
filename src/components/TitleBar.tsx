import { X, Minus, Square } from 'lucide-react';
import { getCurrentWindow } from '@tauri-apps/api/window';

export function TitleBar() {
  const appWindow = getCurrentWindow();

  const handleMinimize = () => {
    appWindow.minimize();
  };

  const handleMaximize = () => {
    appWindow.toggleMaximize();
  };

  const handleClose = () => {
    appWindow.close();
  };

  return (
    <div
      data-tauri-drag-region
      className="h-8 bg-macos-bg border-b border-macos-separator flex items-center justify-between px-4 select-none"
    >
      <div className="flex items-center gap-2">
        <div className="flex items-center gap-2">
          <button
            onClick={handleClose}
            className="w-3 h-3 rounded-full bg-[#FF5F57] hover:bg-[#FF5F57]/80 transition-colors"
            aria-label="Close"
          />
          <button
            onClick={handleMinimize}
            className="w-3 h-3 rounded-full bg-[#FEBC2E] hover:bg-[#FEBC2E]/80 transition-colors"
            aria-label="Minimize"
          />
          <button
            onClick={handleMaximize}
            className="w-3 h-3 rounded-full bg-[#28C840] hover:bg-[#28C840]/80 transition-colors"
            aria-label="Maximize"
          />
        </div>
      </div>

      <div className="absolute left-1/2 -translate-x-1/2">
        <h1 className="text-[13px] font-semibold text-macos-text-primary">
          Y&L Marketing - 중앙 관제
        </h1>
      </div>

      <div className="w-16" /> {/* Spacer for balance */}
    </div>
  );
}

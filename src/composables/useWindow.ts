import { ref } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';

export function useWindow() {
    const appWindow = getCurrentWindow();
    const isMaximized = ref(false);

    async function minimizeWindow() {
        try {
            await appWindow.minimize();
        } catch (e) {
            console.error("minimize error:", e);
        }
    }

    async function toggleMaximize() {
        try {
            const isMax = await appWindow.isMaximized();
            if (isMax) {
                await appWindow.unmaximize();
            } else {
                await appWindow.maximize();
            }
            await updateMaximizeState();
        } catch (e) {
            console.error("maximize error:", e);
        }
    }

    async function updateMaximizeState() {
        try {
            isMaximized.value = await appWindow.isMaximized();
        } catch (e) {
            console.error("Failed to check maximized state:", e);
        }
    }

    async function closeWindow() {
        try {
            await appWindow.close();
        } catch (e) {
            console.error("close error:", e);
        }
    }

    return {
        isMaximized,
        minimizeWindow,
        toggleMaximize,
        updateMaximizeState,
        closeWindow,
        appWindow
    };
}

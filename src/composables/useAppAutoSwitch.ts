import { ref, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

export interface AppProfileMap {
    [appName: string]: number; // Maps app name (e.g. "code") to profile index (1-based)
}

export function useAppAutoSwitch() {
    const activeApp = ref<string>("");
    const activeProfileIndex = ref<number>(1);
    const profileMap = ref<AppProfileMap>({});
    const isAutoSwitchEnabled = ref(true);

    let pollingInterval: number | null = null;

    // Load map from local storage
    function loadMap() {
        const saved = localStorage.getItem('appProfileMap');
        if (saved) {
            profileMap.value = JSON.parse(saved);
        }
    }

    function saveMap() {
        localStorage.setItem('appProfileMap', JSON.stringify(profileMap.value));
    }

    function setProfileForApp(appName: string, profileIndex: number) {
        profileMap.value[appName] = profileIndex;
        saveMap();
    }

    async function checkActiveApp() {
        try {
            const appName = await invoke<string>('get_active_app');
            if (appName && appName !== activeApp.value) {
                activeApp.value = appName;

                // Auto switch logic
                if (isAutoSwitchEnabled.value && profileMap.value[appName]) {
                    const targetProfile = profileMap.value[appName];
                    console.log(`[AutoProfile] Switching to profile ${targetProfile} for ${appName}`);
                    // TODO: Implement actual profile switching when opcode is known
                    // await invoke('akko_set_profile', { profile: targetProfile });
                    activeProfileIndex.value = targetProfile;
                }
            }
        } catch (e) {
            console.error("Error checking active app:", e);
        }
    }

    function startPolling() {
        if (pollingInterval) return;
        loadMap();
        pollingInterval = window.setInterval(checkActiveApp, 1000); // Check every second
    }

    function stopPolling() {
        if (pollingInterval) {
            clearInterval(pollingInterval);
            pollingInterval = null;
        }
    }

    onMounted(() => {
        startPolling();
    });

    onUnmounted(() => {
        stopPolling();
    });

    return {
        activeApp,
        activeProfileIndex,
        profileMap,
        isAutoSwitchEnabled,
        setProfileForApp,
        checkActiveApp
    };
}

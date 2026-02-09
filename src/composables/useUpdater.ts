import { check } from '@tauri-apps/plugin-updater';
import { getVersion } from '@tauri-apps/api/app';
import { ref, onMounted } from 'vue';
import { relaunch } from '@tauri-apps/plugin-process';

export function useUpdater() {
    const updateAvailable = ref(false);
    const updateVersion = ref('');
    const currentVersion = ref('');
    const updateStatus = ref('');
    const isChecking = ref(false);
    const error = ref('');

    onMounted(async () => {
        try {
            currentVersion.value = await getVersion();
        } catch (e) {
            console.error('Failed to get app version:', e);
            currentVersion.value = 'Unknown';
        }
    });

    async function checkForUpdates() {
        isChecking.value = true;
        error.value = '';
        updateStatus.value = 'Checking for updates...';

        try {
            const update = await check();

            if (update) {
                console.log(`found update ${update.version} from ${update.date} with notes ${update.body}`);
                updateAvailable.value = true;
                updateVersion.value = update.version;
                updateStatus.value = `Update available: v${update.version}`;
                return update;
            } else {
                updateAvailable.value = false;
                updateStatus.value = 'You are on the latest version.';
                return null;
            }
        } catch (e: any) {
            console.error(e);
            error.value = e.message || 'Failed to check for updates';
            updateStatus.value = 'Error checking for updates.';
            return null;
        } finally {
            isChecking.value = false;
        }
    }

    async function installUpdate() {
        try {
            const update = await check();
            if (!update) return;

            updateStatus.value = 'Downloading update...';
            let downloaded = 0;
            let contentLength = 0;

            await update.downloadAndInstall((event) => {
                switch (event.event) {
                    case 'Started':
                        contentLength = event.data.contentLength || 0;
                        console.log(`started downloading ${contentLength} bytes`);
                        break;
                    case 'Progress':
                        downloaded += event.data.chunkLength;
                        console.log(`downloaded ${downloaded} from ${contentLength}`);
                        break;
                    case 'Finished':
                        console.log('download finished');
                        updateStatus.value = 'Download finished. Installing...';
                        break;
                }
            });

            updateStatus.value = 'Update installed. Restarting...';
            await relaunch();
        } catch (e: any) {
            console.error(e);
            error.value = e.message || 'Failed to install update';
            updateStatus.value = 'Error installing update.';
        }
    }

    return {
        updateAvailable,
        updateVersion,
        currentVersion,
        updateStatus,
        isChecking,
        error,
        checkForUpdates,
        installUpdate
    };
}

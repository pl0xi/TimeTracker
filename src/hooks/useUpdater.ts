import { check } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";
import { ask } from "@tauri-apps/plugin-dialog";

export async function checkForUpdates(silent = false): Promise<boolean> {
  try {
    const update = await check();

    if (!update) {
      if (!silent) {
        console.log("No update available");
      }
      return false;
    }

    console.log(`Update available: ${update.version}`);

    const yes = await ask(
      `A new version (${update.version}) is available. Would you like to install it now?`,
      {
        title: "Update Available",
        kind: "info",
        okLabel: "Install",
        cancelLabel: "Later",
      }
    );

    if (yes) {
      console.log("Installing update...");

      // Show download progress could be added here
      await update.downloadAndInstall((event) => {
        switch (event.event) {
          case "Started":
            console.log(`Download started, size: ${event.data.contentLength}`);
            break;
          case "Progress":
            console.log(`Downloaded ${event.data.chunkLength} bytes`);
            break;
          case "Finished":
            console.log("Download finished");
            break;
        }
      });

      console.log("Update installed, relaunching...");
      await relaunch();
      return true;
    }

    return false;
  } catch (error) {
    console.error("Error checking for updates:", error);
    return false;
  }
}

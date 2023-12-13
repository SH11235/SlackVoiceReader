import { invoke } from "@tauri-apps/api/tauri";
import { FormEvent, useEffect, useState } from "react";
import styles from "./SettingsForm.module.css";

export interface Settings {
  slackToken: string;
  threadUrl: string;
  voicevoxUrl: string;
  speakerStyleId: string;
}

function SettingsForm() {
  const [settings, setSettings] = useState<Settings>({
    slackToken: "",
    threadUrl: "",
    voicevoxUrl: "",
    speakerStyleId: "",
  });
  const [isFormChanged, setFormChanged] = useState(false);

  useEffect(() => {
    const loadSettings = async () => {
      try {
        const settings = await invoke<Settings>("load_settings");
        console.log("Loaded settings:", settings);
        setSettings({
          slackToken: settings.slackToken || "",
          threadUrl: settings.threadUrl || "",
          voicevoxUrl: settings.voicevoxUrl || "",
          speakerStyleId: settings.speakerStyleId || "",
        });
        setFormChanged(false);
      } catch (err) {
        console.error("Failed to load settings:", err);
      }
    };

    loadSettings();
  }, []);

  const handleSubmit = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    invoke("save_settings", { settings });
    setFormChanged(false);
  };

  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setFormChanged(true);
    setSettings({
      ...settings,
      [event.target.name]: event.target.value,
    });
  };

  return (
    <form onSubmit={handleSubmit} className={styles.settingsForm}>
      <label className={styles.labelText}>
        Slack Token:
        <br />
        <input
          type="text"
          name="slackToken"
          value={settings.slackToken}
          onChange={handleChange}
          className={styles.inputField}
        />
      </label>
      <label className={styles.labelText}>
        Thread URL:
        <br />
        <input
          type="text"
          name="threadUrl"
          value={settings.threadUrl}
          onChange={handleChange}
          className={styles.inputField}
        />
      </label>
      <label className={styles.labelText}>
        VoiceVox URL:
        <br />
        <input
          type="text"
          name="voicevoxUrl"
          value={settings.voicevoxUrl}
          onChange={handleChange}
          className={styles.inputField}
        />
      </label>
      <label className={styles.labelText}>
        Speaker Style ID:
        <br />
        <input
          type="text"
          name="speakerStyleId"
          value={settings.speakerStyleId}
          onChange={handleChange}
          className={styles.inputField}
        />
      </label>
      <input
        type="submit"
        value="Save"
        disabled={!isFormChanged}
        className={styles.submitButton}
      />
    </form>
  );
}

export default SettingsForm;

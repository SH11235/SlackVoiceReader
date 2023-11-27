import { invoke } from "@tauri-apps/api/tauri";
import { FormEvent, useEffect, useState } from "react";

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
      } catch (err) {
        console.error("Failed to load settings:", err);
      }
    };

    loadSettings();
  }, []);

  const handleSubmit = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    invoke("save_settings", { settings });
  };

  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setSettings({
      ...settings,
      [event.target.name]: event.target.value,
    });
  };

  return (
    <>
      <form onSubmit={handleSubmit}>
        <label>
          Slack Token:
          <input
            type="text"
            name="slackToken"
            value={settings.slackToken}
            onChange={handleChange}
          />
        </label>
        <label>
          Thread URL:
          <input
            type="text"
            name="threadUrl"
            value={settings.threadUrl}
            onChange={handleChange}
          />
        </label>
        <label>
          VoiceVox URL:
          <input
            type="text"
            name="voicevoxUrl"
            value={settings.voicevoxUrl}
            onChange={handleChange}
          />
        </label>
        <label>
          Speaker Style ID:
          <input
            type="text"
            name="speakerStyleId"
            value={settings.speakerStyleId}
            onChange={handleChange}
          />
        </label>
        <input type="submit" value="Save" />
      </form>
    </>
  );
}

export default SettingsForm;

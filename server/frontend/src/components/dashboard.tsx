import { sendDashboardConfig } from '@/lib/api/send-dashboard-config';
import { Button } from './ui/button';
import { ColorPicker } from './ui/color-picker';
import {
  Select,
  SelectContent,
  SelectTrigger,
  SelectValue,
  SelectItem,
} from './ui/select';
import { Slider } from './ui/slider';
import WidgetConfigCard from './widget-config-card';
import useAppStore from '@/lib/store/store';
import { toast } from 'sonner';

const Dashboard = () => {
  const accentColor = useAppStore((state) => state.accentColor);
  const setAccentColor = useAppStore((state) => state.setAccentColor);
  const orientation = useAppStore((state) => state.orientation);
  const setOrientation = useAppStore((state) => state.setOrientation);
  const theme = useAppStore((state) => state.theme);
  const setTheme = useAppStore((state) => state.setTheme);
  const charactersPerSecond = useAppStore((state) => state.charactersPerSecond);
  const setCharactersPerSecond = useAppStore(
    (state) => state.setCharactersPerSecond
  );

  const leftWidget = useAppStore((state) => state.leftWidget);
  const centerWidget = useAppStore((state) => state.centerWidget);
  const rightWidget = useAppStore((state) => state.rightWidget);

  const uploadConfig = async () => {
    if (
      await sendDashboardConfig({
        leftWidget,
        centerWidget,
        rightWidget,
        theme,
        orientation,
        accentColor,
        charactersPerSecond,
      })
    ) {
      toast.success('Config uploaded successfully', { position: 'top-right' });
    } else {
      toast.error('Failed to upload config', { position: 'top-right' });
    }
  };

  return (
    <div className="rounded-lg p-4 drop-shadow-lg">
      <h1 className="text-4xl font-bold">Dashboard</h1>
      <p className="text-lg mt-4">
        This is your dashboard. You can customize your default view or change
        the theme of the app.
      </p>
      <h1 className="text-2xl font-bold mt-8">Start view</h1>
      <div className="flex flex-col gap-4 mt-4">
        <WidgetConfigCard widgetView="left" />
        <WidgetConfigCard widgetView="center" />
        <WidgetConfigCard widgetView="right" />
      </div>
      <h1 className="text-2xl font-bold mt-8">Theme settings</h1>
      <div className="flex flex-col gap-4 mt-4">
        <p>Theme</p>
        <Select value={theme} onValueChange={setTheme}>
          <SelectTrigger>
            <SelectValue placeholder="Select theme" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="light">Light</SelectItem>
            <SelectItem value="dark">Dark</SelectItem>
          </SelectContent>
        </Select>
        <p>Orientation</p>
        <Select value={orientation} onValueChange={setOrientation}>
          <SelectTrigger>
            <SelectValue placeholder="Select orientation" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="horizontal">Horizontal</SelectItem>
            <SelectItem value="vertical">Vertical</SelectItem>
          </SelectContent>
        </Select>
        <p>Accent color</p>
        <ColorPicker value={accentColor} onChange={setAccentColor} />
      </div>
      <h1 className="text-2xl font-bold mt-8">Message settings</h1>
      <div className="flex flex-col gap-4 mt-4">
        <p>Characters per second</p>
        <Slider
          min={1}
          max={10}
          step={1}
          value={[charactersPerSecond]}
          onValueChange={([value]) => setCharactersPerSecond(value)}
        />
      </div>
      <Button className="mt-8" onClick={uploadConfig}>
        Upload config
      </Button>
    </div>
  );
};

export default Dashboard;

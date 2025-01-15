import { HTMLAttributes, useMemo } from 'react';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from './ui/card';
import {
  Ban,
  CircleDollarSign,
  Clock,
  CloudDrizzle,
  Music,
} from 'lucide-react';
import WidgetButton from './widget-button';
import { Widget } from '@/lib/types';
import useAppStore from '@/lib/store/store';

interface Props extends HTMLAttributes<HTMLDivElement> {
  widgetView: 'left' | 'center' | 'right';
}

const WidgetConfigCard = ({ widgetView, ...props }: Props) => {
  const setLeftWidget = useAppStore((state) => state.setLeftWidget);
  const setCenterWidget = useAppStore((state) => state.setCenterWidget);
  const setRightWidget = useAppStore((state) => state.setRightWidget);

  const leftWidget = useAppStore((state) => state.leftWidget);
  const centerWidget = useAppStore((state) => state.centerWidget);
  const rightWidget = useAppStore((state) => state.rightWidget);

  const widgetViewFormatted = useMemo(() => {
    return widgetView.charAt(0).toUpperCase() + widgetView.slice(1);
  }, [widgetView]);

  const handleWidgetClick = (widget: Widget) => {
    switch (widgetView) {
      case 'left':
        setLeftWidget(widget);
        break;
      case 'center':
        setCenterWidget(widget);
        break;
      case 'right':
        setRightWidget(widget);
        break;
    }
  };

  const getSelectedWidget = () => {
    switch (widgetView) {
      case 'left':
        return leftWidget;
      case 'center':
        return centerWidget;
      case 'right':
        return rightWidget;
    }
  };

  return (
    <Card className="" {...props}>
      <CardHeader>
        <CardTitle>{widgetViewFormatted} view</CardTitle>
        <CardDescription>
          Select the widget you want to see on the {widgetView} side of the IoT
          Screen.
        </CardDescription>
      </CardHeader>
      <CardContent>
        <div className="flex gap-4">
          <WidgetButton
            className="w-16 h-16"
            widgetName="None"
            selected={Widget.None === getSelectedWidget()}
            icon={<Ban />}
            onClick={() => handleWidgetClick(Widget.None)}
          />
          <WidgetButton
            className="w-16 h-16"
            widgetName="Spotify: Currently playing"
            selected={Widget.Spotify === getSelectedWidget()}
            icon={<Music />}
            onClick={() => handleWidgetClick(Widget.Spotify)}
          />
          <WidgetButton
            className="w-16 h-16"
            widgetName="Current weather"
            selected={Widget.Weather === getSelectedWidget()}
            icon={<CloudDrizzle />}
            onClick={() => handleWidgetClick(Widget.Weather)}
          />
          <WidgetButton
            className="w-16 h-16"
            widgetName="XTB Portfolio"
            selected={Widget.Xtb === getSelectedWidget()}
            icon={<CircleDollarSign />}
            onClick={() => handleWidgetClick(Widget.Xtb)}
          />
          <WidgetButton
            className="w-16 h-16"
            widgetName="Clock"
            selected={Widget.Clock === getSelectedWidget()}
            icon={<Clock />}
            onClick={() => handleWidgetClick(Widget.Clock)}
          />
        </div>
      </CardContent>
    </Card>
  );
};

export default WidgetConfigCard;

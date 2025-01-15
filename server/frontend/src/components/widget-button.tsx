import { Button } from './ui/button';
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from './ui/tooltip';

interface Props extends React.HTMLAttributes<HTMLButtonElement> {
  widgetName: string;
  selected: boolean;
  icon: JSX.Element;
}

const WidgetButton = ({ widgetName, selected, icon, ...props }: Props) => {
  return (
    <TooltipProvider>
      <Tooltip>
        <TooltipTrigger asChild>
          <Button variant={selected ? 'secondary' : 'ghost'} {...props}>
            {icon}
          </Button>
        </TooltipTrigger>
        <TooltipContent>{widgetName}</TooltipContent>
      </Tooltip>
    </TooltipProvider>
  );
};

export default WidgetButton;

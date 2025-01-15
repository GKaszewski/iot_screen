import { cn } from '@/lib/utils';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from './ui/card';
import { Input } from './ui/input';
import { Label } from './ui/label';
import { Button } from './ui/button';
import { toast } from 'sonner';

interface Props extends React.HTMLAttributes<HTMLDivElement> {
  appName: string;

  clientId: string;
  clientSecret: string;
  redirectUri: string;
  authorizeUrl: string;
  getTokenUrl: string;

  onClientIdChange: (clientId: string) => void;
  onClientSecretChange: (clientSecret: string) => void;
  onRedirectUriChange: (redirectUri: string) => void;
  onAuthorizeUrlChange: (authorizeUrl: string) => void;
  onGetTokenUrlChange: (getTokenUrl: string) => void;
}

const OAuthClientConfig = ({
  appName,
  clientId,
  clientSecret,
  redirectUri,
  authorizeUrl,
  getTokenUrl,
  onClientIdChange,
  onClientSecretChange,
  onRedirectUriChange,
  onAuthorizeUrlChange,
  onGetTokenUrlChange,
  ...props
}: Props) => {
  const handleClientIdChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    onClientIdChange(e.target.value);
  };

  const handleClientSecretChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    onClientSecretChange(e.target.value);
  };

  const handleRedirectUriChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    onRedirectUriChange(e.target.value);
  };

  const handleAuthorizeUrlChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    onAuthorizeUrlChange(e.target.value);
  };

  const handleGetTokenUrlChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    onGetTokenUrlChange(e.target.value);
  };

  const handleAuthorize = () => {
    toast('Opening authorization URL in a new tab');
    if (!authorizeUrl) {
      return;
    }

    window.open(authorizeUrl);
  };

  return (
    <Card className={cn('w-[450px]', props.className)}>
      <CardHeader>
        <CardTitle>{appName}</CardTitle>
        <CardDescription>Configure your OAuth client</CardDescription>
      </CardHeader>
      <CardContent>
        <div className="grid grid-cols-2 gap-4">
          <Label>Client ID</Label>
          <Input
            placeholder="Client ID"
            value={clientId}
            onChange={handleClientIdChange}
          />
          <Label>Client Secret</Label>
          <Input
            placeholder="Client Secret"
            value={clientSecret}
            onChange={handleClientSecretChange}
          />
          <Label>Redirect URI</Label>
          <Input
            placeholder="Redirect URI"
            value={redirectUri}
            onChange={handleRedirectUriChange}
          />
          <Label>Get Token URL</Label>
          <Input
            placeholder="Get Token URL"
            value={getTokenUrl}
            onChange={handleGetTokenUrlChange}
          />
        </div>

        <div className="flex flex-col gap-4 mt-4">
          <p className="leading-7 [&:not(:first-child)]:mt-6">
            To authorize your OAuth client, you need to provide the
            Authorization URL. This is the URL where the user will be redirected
            to authorize your app.
          </p>
          <Label>Authorization URL</Label>
          <Input value={authorizeUrl} onChange={handleAuthorizeUrlChange} />
          <Button onClick={handleAuthorize} type="button">
            Authorize
          </Button>
        </div>
      </CardContent>
    </Card>
  );
};

export default OAuthClientConfig;

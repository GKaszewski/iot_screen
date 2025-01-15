import useAppStore from '@/lib/store/store';
import { Button } from './ui/button';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from './ui/card';
import { Input } from './ui/input';
import { Label } from './ui/label';
import { sendXtbCredentials } from '@/lib/api/send-xtb-credentials';
import { toast } from 'sonner';

const XtbLoginScreen = () => {
  const xtbEmail = useAppStore((state) => state.xtbEmail);
  const xtbPassword = useAppStore((state) => state.xtbPassword);
  const setXtbEmail = useAppStore((state) => state.setXtbEmail);
  const setXtbPassword = useAppStore((state) => state.setXtbPassword);

  const handleSaveCredentials = async () => {
    if (!xtbEmail || !xtbPassword) {
      toast.info('Please enter your XTB credentials', {
        position: 'top-right',
      });
      return;
    }

    if (await sendXtbCredentials({ email: xtbEmail, password: xtbPassword })) {
      toast.success('Credentials saved successfully', {
        position: 'top-right',
      });
    } else {
      toast.error('Failed to save credentials', { position: 'top-right' });
    }
  };

  return (
    <Card className="w-[450px] h-fit p-4">
      <CardHeader>
        <CardTitle>XTB Login</CardTitle>
        <CardDescription>
          Enter your XTB credentials to see your account information.
        </CardDescription>
      </CardHeader>
      <CardContent>
        <div className="flex flex-col gap-4 mb-4">
          <Label>Email</Label>
          <Input
            type="email"
            value={xtbEmail}
            onChange={(e) => setXtbEmail(e.target.value)}
          />
          <Label>Password</Label>
          <Input
            type="password"
            value={xtbPassword}
            onChange={(e) => setXtbPassword(e.target.value)}
          />
        </div>
        <Button onClick={handleSaveCredentials}>Save credentials</Button>
      </CardContent>
    </Card>
  );
};

export default XtbLoginScreen;

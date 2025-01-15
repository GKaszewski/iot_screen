import useQueryParams from '@/hooks/use-query-params';
import { sendOAuth2Code } from '@/lib/api/send-oauth-code';
import useAppStore from '@/lib/store/store';
import { useEffect } from 'react';
import { toast } from 'sonner';

const OAuthTokenExchanger = () => {
  const queryParams = useQueryParams();

  const spotifyCallbackUrl = useAppStore((state) => state.spotifyCallbackUrl);
  const spotifyClientId = useAppStore((state) => state.spotifyClientId);
  const spotifyClientSecret = useAppStore((state) => state.spotifyClientSecret);
  const spotifyRedirectUrl = useAppStore((state) => state.spotifyCallbackUrl);
  const spotifyGetTokenUrl = useAppStore((state) => state.spotifyGetTokenUrl);

  const setSpotifyCode = useAppStore((state) => state.setSpotifyCode);

  const mapPathToOAuthTokens = async () => {
    const fullUrl = window.location.href;
    const url = new URL(fullUrl);
    const urlWithoutSearch = url.origin + url.pathname;

    switch (urlWithoutSearch) {
      case spotifyCallbackUrl: {
        setSpotifyCode(queryParams.get('code') || '');
        const code = queryParams.get('code');
        if (code) {
          const successful = await sendOAuth2Code({
            code,
            appName: 'spotify',
            clientId: spotifyClientId,
            clientSecret: spotifyClientSecret,
            redirectUri: spotifyRedirectUrl,
            getTokenUrl: spotifyGetTokenUrl,
          });
          if (successful) {
            toast('Successfully exchanged code for tokens');
          } else {
            toast('Failed to exchange code for tokens');
          }
        }
        break;
      }
    }
  };

  useEffect(() => {
    mapPathToOAuthTokens();
  }, [queryParams]);

  return <></>;
};

export default OAuthTokenExchanger;

import useAppStore from '@/lib/store/store';
import OAuthClientConfig from './oauth-client-config';

const OAuthConfiguration = () => {
  const spotifyClientId = useAppStore((state) => state.spotifyClientId);
  const spotifyClientSecret = useAppStore((state) => state.spotifyClientSecret);
  const spotifyAuthorizeUrl = useAppStore((state) => state.spotifyAuthorizeUrl);
  const spotifyCallbackUrl = useAppStore((state) => state.spotifyCallbackUrl);
  const spotifyGetTokenUrl = useAppStore((state) => state.spotifyGetTokenUrl);

  const setSpotifyClientId = useAppStore((state) => state.setSpotifyClientId);
  const setSpotifyClientSecret = useAppStore(
    (state) => state.setSpotifyClientSecret
  );
  const setSpotifyAuthorizeUrl = useAppStore(
    (state) => state.setSpotifyAuthorizeUrl
  );
  const setSpotifyCallbackUrl = useAppStore(
    (state) => state.setSpotifyCallbackUrl
  );
  const setSpotifyGetTokenUrl = useAppStore(
    (state) => state.setSpotifyGetTokenUrl
  );

  return (
    <section className="flex flex-col gap-4">
      <h1 className="text-4xl font-bold mt-8">Your OAuth2 integrations</h1>
      <OAuthClientConfig
        appName="Spotify"
        authorizeUrl={spotifyAuthorizeUrl}
        clientId={spotifyClientId}
        clientSecret={spotifyClientSecret}
        redirectUri={spotifyCallbackUrl}
        getTokenUrl={spotifyGetTokenUrl}
        onAuthorizeUrlChange={setSpotifyAuthorizeUrl}
        onClientIdChange={setSpotifyClientId}
        onClientSecretChange={setSpotifyClientSecret}
        onRedirectUriChange={setSpotifyCallbackUrl}
        onGetTokenUrlChange={setSpotifyGetTokenUrl}
      />
    </section>
  );
};

export default OAuthConfiguration;

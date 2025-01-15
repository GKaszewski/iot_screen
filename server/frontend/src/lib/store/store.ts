import { StoreApi, create } from 'zustand';
import { persist } from 'zustand/middleware';

export interface AppSlice {
  spotifyCode: string;
  spotifyCallbackUrl: string;
  spotifyClientId: string;
  spotifyClientSecret: string;
  spotifyAuthorizeUrl: string;
  spotifyGetTokenUrl: string;

  setSpotifyCode: (code: string) => void;
  setSpotifyCallbackUrl: (url: string) => void;
  setSpotifyClientId: (clientId: string) => void;
  setSpotifyClientSecret: (clientSecret: string) => void;
  setSpotifyAuthorizeUrl: (authorizeUrl: string) => void;
  setSpotifyGetTokenUrl: (getTokenUrl: string) => void;
}

export type StoreState = AppSlice;

export type StoreSlice<T> = (
  set: StoreApi<StoreState>['setState'],
  get: StoreApi<StoreState>['getState']
) => T;

const createAppSlice: StoreSlice<AppSlice> = (set) => ({
  spotifyCode: '',
  spotifyCallbackUrl: '',
  spotifyClientId: '',
  spotifyClientSecret: '',
  spotifyAuthorizeUrl: '',
  spotifyGetTokenUrl: '',

  setSpotifyCode: (code: string) => {
    set({ spotifyCode: code });
  },
  setSpotifyCallbackUrl: (url: string) => {
    set({ spotifyCallbackUrl: url });
  },
  setSpotifyClientId: (clientId: string) => {
    set({ spotifyClientId: clientId });
  },
  setSpotifyClientSecret: (clientSecret: string) => {
    set({ spotifyClientSecret: clientSecret });
  },
  setSpotifyAuthorizeUrl: (authorizeUrl: string) => {
    set({ spotifyAuthorizeUrl: authorizeUrl });
  },
  setSpotifyGetTokenUrl: (getTokenUrl: string) => {
    set({ spotifyGetTokenUrl: getTokenUrl });
  },
});

export const createPartializedState = (state: StoreState) => ({
  ...state,
});

const useAppStore = create<StoreState>()(
  persist(
    (set, get) => ({
      ...createAppSlice(set, get),
    }),
    {
      name: 'app-store',
      partialize: (state) => createPartializedState(state),
      version: 1,
    }
  )
);

export default useAppStore;

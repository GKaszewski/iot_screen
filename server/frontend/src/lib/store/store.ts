import { StoreApi, create } from 'zustand';
import { persist } from 'zustand/middleware';
import { Widget } from '../types';

export interface AppSlice {
  spotifyCode: string;
  spotifyCallbackUrl: string;
  spotifyClientId: string;
  spotifyClientSecret: string;
  spotifyAuthorizeUrl: string;
  spotifyGetTokenUrl: string;

  xtbUserId: string;
  xtbPassword: string;

  leftWidget: Widget;
  centerWidget: Widget;
  rightWidget: Widget;

  theme: 'light' | 'dark';
  orientation: 'horizontal' | 'vertical';
  accentColor: string;
  charactersPerSecond: number;

  setSpotifyCode: (code: string) => void;
  setSpotifyCallbackUrl: (url: string) => void;
  setSpotifyClientId: (clientId: string) => void;
  setSpotifyClientSecret: (clientSecret: string) => void;
  setSpotifyAuthorizeUrl: (authorizeUrl: string) => void;
  setSpotifyGetTokenUrl: (getTokenUrl: string) => void;

  setXtbUserId: (email: string) => void;
  setXtbPassword: (password: string) => void;

  setLeftWidget: (widget: Widget) => void;
  setCenterWidget: (widget: Widget) => void;
  setRightWidget: (widget: Widget) => void;

  setTheme: (theme: 'light' | 'dark') => void;
  setOrientation: (orientation: 'horizontal' | 'vertical') => void;
  setAccentColor: (color: string) => void;
  setCharactersPerSecond: (cps: number) => void;
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

  xtbUserId: '',
  xtbPassword: '',

  leftWidget: Widget.None,
  centerWidget: Widget.None,
  rightWidget: Widget.None,

  theme: 'light',
  orientation: 'horizontal',
  accentColor: '#22C55E',
  charactersPerSecond: 2,

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

  setXtbUserId: (email: string) => {
    set({ xtbUserId: email });
  },
  setXtbPassword: (password: string) => {
    set({ xtbPassword: password });
  },

  setLeftWidget: (widget: Widget) => {
    set({ leftWidget: widget });
  },
  setCenterWidget: (widget: Widget) => {
    set({ centerWidget: widget });
  },
  setRightWidget: (widget: Widget) => {
    set({ rightWidget: widget });
  },

  setTheme: (theme: 'light' | 'dark') => {
    set({ theme });
  },
  setOrientation: (orientation: 'horizontal' | 'vertical') => {
    set({ orientation });
  },
  setAccentColor: (color: string) => {
    set({ accentColor: color });
  },
  setCharactersPerSecond: (cps: number) => {
    set({ charactersPerSecond: cps });
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

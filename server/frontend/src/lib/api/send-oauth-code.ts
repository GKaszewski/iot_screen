import { base } from './base';

export type SendOAuth2CodePayload = {
  code: string;
  appName: string;
  clientSecret: string;
  clientId: string;
  redirectUri: string;
  getTokenUrl: string;
};

export const sendOAuth2Code = async (payload: SendOAuth2CodePayload) => {
  try {
    const response = await base.post('/oauth2/code', payload, {
      headers: {
        'Content-Type': 'application/json',
      },
    });
    if (response.status === 200) {
      return true;
    } else {
      console.error('Unexpected response:', response);
      return false;
    }
  } catch (error) {
    console.error('Error during POST request:', error);
    return false;
  }
};

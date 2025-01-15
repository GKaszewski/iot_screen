import { base } from './base';

export type SendXtbCredentialsPayload = {
  email: string;
  password: string;
};

export const sendXtbCredentials = async (
  payload: SendXtbCredentialsPayload
) => {
  try {
    const response = await base.post('/xtb/credentials', payload, {
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

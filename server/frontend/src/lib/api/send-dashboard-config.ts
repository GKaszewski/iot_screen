import { Widget } from '../types';
import { base } from './base';

export type SendDashboardConfigPayload = {
  leftWidget: Widget;
  centerWidget: Widget;
  rightWidget: Widget;
  theme: 'light' | 'dark';
  orientation: 'horizontal' | 'vertical';
  accentColor: string;
  charactersPerSecond: number;
};

export const sendDashboardConfig = async (
  payload: SendDashboardConfigPayload
) => {
  try {
    const response = await base.post('/dashboard/config', payload, {
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

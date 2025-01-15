import axios from 'axios';

const BASE_URL = import.meta.env.VITE_BASE_URL;
if (!BASE_URL) {
  throw new Error('VITE_BASE_URL is not set');
}

export const base = axios.create({
  baseURL: BASE_URL,
  withCredentials: false,
});

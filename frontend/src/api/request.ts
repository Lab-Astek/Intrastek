import axios from "axios";
import { AxiosRequestConfig, AxiosError } from "axios";
import { env } from "process";

const API_URL: string = env.API_URL || "http://localhost";
const API_PORT: string = env.API_PORT || "8000";

async function request(method: string, endpoint: string, data: any = {}) {
  let config = {
    method: method,
    maxBodyLength: Infinity,
    url: `${API_URL}:${API_PORT}/${endpoint}`,
    headers: {},
    data: data,
  };

  return axios.request(config);
}

export async function log_auth(method: string, endpoint: string, token: string) {
  let config = {
    method: method,
    maxBodyLength: Infinity,
    url: `${API_URL}:${API_PORT}/${endpoint}`,
    headers: {
      'Authorization': `Bearer ${token}`,
      'Accept': 'application/json'
    }
  };
  console.log('Sending request to backend with config:', config);

  try {
    const response = await axios.request(config);
    console.log('Response from backend:', response);
    return response;
  } catch (error: unknown) {
    if (axios.isAxiosError(error)) {
      if (error.response) {
        console.error('Error response from backend:', error.response.status, error.response.data);
      } else if (error.request) {
        console.error('No response received from backend:', error.request);
      } else {
        console.error('Error in setting up request:', error.message);
      }
    } else {
      console.error('Unknown error:', error);
    }
    throw error;
  }
}

export async function post(endpoint: string, data: any) {
  return request("POST", endpoint, { data: data });
}

export async function get(endpoint: string) {
  return request("GET", endpoint);
}

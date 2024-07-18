import { AccountInfo } from "@azure/msal-browser";
import axios, { AxiosResponse } from "axios";
import { AxiosRequestConfig, AxiosError } from "axios";
import { env } from "process";

const API_URL: string = env.API_URL || "http://localhost";
const API_PORT: string = env.API_PORT || "8000";

async function request<T>(
  method: string,
  endpoint: string,
  user: AccountInfo | null,
  data: any = {}
): Promise<AxiosResponse<T>> {
  let config = {
    method: method,
    maxBodyLength: Infinity,
    url: `${API_URL}:${API_PORT}/${endpoint}`,
    headers: {
      Authorization: `Bearer ${user?.idToken}`,
      "X-email": user?.username,
    },
    data: data,
  };

  return axios.request<T>(config);
}

export async function log_auth(
  method: string,
  endpoint: string,
  token: string
) {
  let config = {
    method: method,
    maxBodyLength: Infinity,
    url: `${API_URL}:${API_PORT}/${endpoint}`,
    headers: {
      Authorization: `Bearer ${token}`,
      Accept: "application/json",
    },
  };
  console.log("Sending request to backend with config:", config);

  try {
    const response = await axios.request(config);
    console.log("Response from backend:", response);
    return response;
  } catch (error: unknown) {
    if (axios.isAxiosError(error)) {
      if (error.response) {
        console.error(
          "Error response from backend:",
          error.response.status,
          error.response.data
        );
      } else if (error.request) {
        console.error("No response received from backend:", error.request);
      } else {
        console.error("Error in setting up request:", error.message);
      }
    } else {
      console.error("Unknown error:", error);
    }
    throw error;
  }
}

export async function post<T>(
  endpoint: string,
  data: any,
  user: AccountInfo | null
): Promise<AxiosResponse<T>> {
  return request<T>("POST", endpoint, user, { data: data });
}

export async function get<T>(
  endpoint: string,
  user: AccountInfo | null
): Promise<AxiosResponse<T>> {
  return request<T>("GET", endpoint, user);
}

export async function ping(user: AccountInfo | null) {
  return request<String>("GET", "ping", user);
}

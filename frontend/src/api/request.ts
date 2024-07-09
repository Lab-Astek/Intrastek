import axios from "axios";
import { env } from "process";

const API_URL: string = env.API_URL || "http://localhost";
const API_PORT: string = env.API_PORT || "8000";

async function request(method: string, endpoint: string, data: any = {}) {
    let config = {
        method: method,
        maxBodyLength: Infinity,
        url: `${API_URL}:${API_PORT}/${endpoint}`,
        headers: {},
        data: data
    };

    return axios.request(config);
}

export async function log_auth(method: string, endpoint: string, token: string)
{
    let config = {
        method: method,
        maxBodyLength: Infinity,
        url: `${API_URL}:${API_PORT}/${endpoint}`,
        headers: {
            'Authorization': `Bearer ${token}`,
            'Accept'       : 'application/json'
        }
    };

    return axios.request(config);
}

export async function get(endpoint: string)
{
    return request("GET", endpoint)
}

export async function post(endpoint: string, data: any = {})
{
    return request("POST", endpoint, data)
}

import axios from "axios";
import { env } from "process";

const API_URL: string = env.API_URL || "http://localhost";
const API_PORT: string = env.API_PORT || "8000";

export default async function request(method: string, endpoint: string, data: any) {
    let config = {
        method: method,
        maxBodyLength: Infinity,
        url: `${API_URL}:${API_PORT}/${endpoint}`,
        headers: {},
        data: data
    };

    return axios.request(config);
}
"use client";

import React from "react";
import { useMsal } from "@azure/msal-react";
import { Button } from "@mui/material";
import { msalConfig, loginRequest } from "../../../authConfig";
import { PublicClientApplication } from "@azure/msal-browser";
import { post } from "../../../api/request";

const msalInstance = new PublicClientApplication(msalConfig);

export default function Login() {
    const { instance } = useMsal();

    function handleLogin() {
        instance.loginPopup(loginRequest).then(response => {
            sendTokenToBackend(response.accessToken);
        }).catch(e => {
            console.error(e);
        });
    }

    async function sendTokenToBackend(token: string) {
        try {
            const response = await post('login', { token });
            console.log('Success:', response.data);
        } catch (error) {
            console.error('Error:', error);
        }
    }

    return (
        <main className="flex min-h-screen flex-col items-center justify-between p-24">
                <Button variant="contained" color="primary" onClick={() => handleLogin("popup")}>
                    Login with Popup
                </Button>
        </main>
    );
}

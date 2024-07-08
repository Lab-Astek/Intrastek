import React from "react";
import { useMsal } from "@azure/msal-react";
import { TextField, Button } from "@mui/material";
import { msalConfig, loginRequest } from "../../../authConfig";
import { PublicClientApplication } from "@azure/msal-browser";
import ButtonWrapper from "@/components/button";

const msalInstance = new PublicClientApplication(msalConfig);

export default function Login() {
    const { instance } = useMsal();

    function handleLogin(loginType: string) {
        if (loginType === "popup") {
            instance.loginPopup(loginRequest).catch(e => {
                console.error(e);
            });
        } else if (loginType === "redirect") {
            instance.loginRedirect(loginRequest).catch(e => {
                console.error(e);
            });
        }
    }

    return (
        <div className="flex flex-col items-center justify-between p-24">
            <TextField id="email" label="Email" variant="standard" />
            <TextField id="password" label="Password" type="password" variant="standard" />
            <ButtonWrapper>
                <Button variant="contained" color="primary" onClick={() => handleLogin("popup")}>
                    Login with Popup
                </Button>
                <Button variant="contained" color="secondary" onClick={() => handleLogin("redirect")}>
                    Login with Redirect
                </Button>
            </ButtonWrapper>
        </div>
    );
}

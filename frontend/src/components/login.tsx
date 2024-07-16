"use client";
import { log_auth } from "@/api/request";
import { loginRequest } from "@/authConfig";
import { useAccount, useIsAuthenticated, useMsal } from "@azure/msal-react";
import { useEffect, useState } from "react";
import { Box } from "@mui/material";
import { InteractionStatus } from "@azure/msal-browser";

function LogoutButton() {
  const { instance } = useMsal();

  const [anchorEl, setAnchorEl] = useState<null | HTMLElement>(null);
  const open = Boolean(anchorEl);

  function handleLogout() {
    setAnchorEl(null);
    instance.logoutPopup();
  }

  return <button onClick={handleLogout}>Logout</button>;
}

function LoginButton() {
  const { instance } = useMsal();

  function handleLogin() {
    instance
      .loginPopup(loginRequest)
      .then((response) => {
        sendTokenToBackend(response.accessToken);
      })
      .catch((e) => {
        console.error(e);
      });
  }

  async function sendTokenToBackend(token: string) {
    try {
      console.log("Sending token to backend...");
      const response = await log_auth("POST", "asteks", token);
      console.log("Success:", response.data);
    } catch (error) {
      console.error("Error:", error);
    }
  }

  return <button onClick={handleLogin}>Login</button>;
}

export function LoginAstekButton() {
  const status = useMsal();
  const isAuthenticated = useIsAuthenticated();

  if (isAuthenticated) {
    return <LogoutButton />;
  } else if (
    status !== InteractionStatus.Startup &&
    status !== InteractionStatus.HandleRedirect
  ) {
    return <LoginButton />;
  } else {
    return null;
  }
}

export const AccountInfo = () => {
  const { accounts } = useMsal();
  const account = useAccount(accounts[0] || {});
  const [name, setName] = useState("");
  const [email, setEmail] = useState("");

  useEffect(() => {
    if (account && account.name) {
      setName(account.name);
      setEmail(account.username);
    }
  }, [account]);

  if (name || email) {
    return (
      <Box>
        <h1>Active account: {name}</h1>
        <h1>Email: {email}</h1>
      </Box>
    );
  } else {
    return null;
  }
};

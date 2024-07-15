"use client";
import { useState, useEffect } from "react";
import { UUID, randomUUID } from "crypto";
import { getAstek } from "../api/asteks";
import { Astek } from "../types/astek";
import ButtonWrapper from "@/components/button";
import Page from "@/components/page";
import SelectWrapper from "@/components/inputs/select";
import { Box } from "@mui/material";

import { MsalProvider, useMsal, useAccount, useIsAuthenticated } from "@azure/msal-react";
import { loginRequest } from "../authConfig";
import { EventType, EventMessage, AuthenticationResult, InteractionStatus } from "@azure/msal-browser";
import { AuthenticatedTemplate, UnauthenticatedTemplate } from "@azure/msal-react";
import { msalInstance } from "./msal/MsGraphApiCall";
import { log_auth } from "../api/request"

const TEST_ID: UUID = "2fdfd8fe-59c0-4a93-9f3b-e0f75110bb1b";

function AstekButton() {
  let [result, setResult] = useState<Astek | undefined>(undefined);

  function handleClick() {
    getAstek(TEST_ID)
      .then((response) => {
        setResult(response.data);
      })
      .catch((error) => {
        console.log(error);
      });
  }

  return <button onClick={handleClick}>Click me {result?.id}</button>;
}

function ActivityCreationPageButton() {
  return (
    <ButtonWrapper>
      <a href="/activity/create">Create an activity</a>
    </ButtonWrapper>
  );
}


function LogoutButton() {
  const { instance } = useMsal();

  const [anchorEl, setAnchorEl] = useState<null | HTMLElement>(null);
  const open = Boolean(anchorEl);

  function handleLogout() {
    setAnchorEl(null);
    instance.logoutPopup();
  }

  return <button onClick={handleLogout}>Logout</button>
}

function LoginButton() {
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
      console.log("Sending token to backend...");
      const response = await log_auth('POST', 'asteks', token);
      console.log('Success:', response.data);
    } catch (error) {
      console.error('Error:', error);
    }
  }

  return <button onClick={handleLogin}>Login</button>
}

function LoginAstekButton() {
  const status = useMsal();
  const isAuthenticated = useIsAuthenticated();


  if (isAuthenticated) {
        return <LogoutButton />;
  } else if (status !== InteractionStatus.Startup && status !== InteractionStatus.HandleRedirect) {
      return <LoginButton />;
  } else {
      return null;
  }
}

const AccountInfo = () => {
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
        <>
          <Box>
            <h1>Active account: {name}</h1>
            <h1>Email: {email}</h1>
          </Box>
        </>
      );
    } else {
      return null;
    }
};

export default function Home() {
  msalInstance.initialize().then(() => {
    const accounts = msalInstance.getAllAccounts();
        if (accounts.length > 0) {
            msalInstance.setActiveAccount(accounts[0]);
            console.log("Active account set to: ", accounts[0])
        }
        msalInstance.addEventCallback((event: EventMessage) => {
          console.log("Event detected: ", event)
          if (event.eventType === EventType.LOGIN_SUCCESS && event.payload) {
              const payload = event.payload as AuthenticationResult;
              const account = payload.account;
              msalInstance.setActiveAccount(account);
              console.log("Active account set to: ", account)
          }
      });
  });

  return (
    <div>
      <Page title="Intrastek">
        <MsalProvider instance={msalInstance}>
          <main className="flex min-h-screen flex-col items-center justify-between p-24">
            <LoginAstekButton />
            <AccountInfo />
            <AstekButton />
          </main>
        </MsalProvider>
      </Page>
    </div>
  );
}

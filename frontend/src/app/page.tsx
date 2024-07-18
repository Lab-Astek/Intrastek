"use client";
import Page from "@/components/page";

import { MsalProvider } from "@azure/msal-react";
import {
  EventType,
  EventMessage,
  AuthenticationResult,
} from "@azure/msal-browser";
import { msalInstance } from "@/components/msal/MsGraphApiCall";
import { AccountInfo, LoginAstekButton } from "@/components/login";

export default function Home() {
  msalInstance.initialize().then(() => {
    const accounts = msalInstance.getAllAccounts();
    if (accounts.length > 0) {
      msalInstance.setActiveAccount(accounts[0]);
      console.log("Active account set to: ", accounts[0]);
    }
    msalInstance.addEventCallback((event: EventMessage) => {
      console.log("Event detected: ", event);
      if (event.eventType === EventType.LOGIN_SUCCESS && event.payload) {
        const payload = event.payload as AuthenticationResult;
        const account = payload.account;
        msalInstance.setActiveAccount(account);
        console.log("Active account set to: ", account);
      }
    });
  });

  return (
    <div>
      <MsalProvider instance={msalInstance}>
        <Page title="Intrastek">
          <LoginAstekButton />
          <AccountInfo />
        </Page>
      </MsalProvider>
    </div>
  );
}

"use client"
import { useEffect, useState } from "react";
import Paper from "@mui/material/Paper";
import { Box } from "@mui/material";
import { MsalAuthenticationTemplate, useMsal, MsalAuthenticationResult } from "@azure/msal-react";
import { InteractionStatus, InteractionType, InteractionRequiredAuthError, AccountInfo } from "@azure/msal-browser";
import { loginRequest } from "../../authConfig";
import { callMsGraph } from "../msal/MsGraphApiCall";
import { AstekData } from "../../types/profile";
import { ProfileData } from "../../components/profile";

const NotFound = () => {
    return (
        <Box component="section" sx={{ p: 2, border: '1px dashed grey' }}>
            No Data
        </Box>
    )
};

const ProfileContent = () => {
    const { instance, inProgress } = useMsal();
    const [astekData, setAstekData] = useState<null|AstekData>(null);

    useEffect(() => {
        if (!astekData && inProgress === InteractionStatus.None) {
            callMsGraph().then(response => setAstekData(response)).catch((e) => {
                if (e instanceof InteractionRequiredAuthError) {
                    instance.acquireTokenRedirect({
                        ...loginRequest,
                        account: instance.getActiveAccount() as AccountInfo
                    });
                }
            });
        }
    }, [inProgress, astekData, instance]);

    return (
        <Paper>
            { astekData ? <ProfileData data={astekData} /> : <NotFound /> }
        </Paper>
    );
};

const ErrorComponent: React.FC<MsalAuthenticationResult> = ({error}) => {
    return (
        <Box component="section" sx={{ p: 2, border: '1px dashed grey' }}>
            An Error Occurred: {error ? error.errorCode : "unknown error"}
        </Box>
    )
}

const Loading = () => {
    return (
        <Box component="section" sx={{ p: 2, border: '1px dashed grey' }}>
            Loading...
        </Box>
    )
};

export default function Home() {
    const authRequest = {
        ...loginRequest
    };

    return (
        <MsalAuthenticationTemplate
            interactionType={InteractionType.Redirect}
            authenticationRequest={authRequest}
            errorComponent={ErrorComponent}
            loadingComponent={Loading}>
            <ProfileContent />
        </MsalAuthenticationTemplate>
      )
};

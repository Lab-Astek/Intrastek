import List from "@mui/material/List";
import Page from "../components/page";
import { AstekData } from "../types/profile";

export const ProfileData: React.FC<{data: AstekData}> = ({data}) => {
    return (
        <Page title={"Astek Profile"} >
            <List>
                <h1>{data.name}</h1>
                <h2>{data.mail}</h2>
            </List>
        </Page>
    );
};

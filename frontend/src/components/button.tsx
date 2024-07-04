import Button from '@mui/material/Button';

export default function BasicButton({ children }: { children: any }) {
    return (
        <Button variant="outlined">{children}</Button>
    );
}

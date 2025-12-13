import { Button } from "@mantine/core";
import { useMyContext } from "../state";

export function Toolbar() {
    const { render } = useMyContext();

    return (<div>
        <Button onClick={() => { void render(); }}>Render</Button>
    </div>);
}

import { Container, Text, Title } from '@mantine/core';
import './App.css'
import { Editor } from '@monaco-editor/react';

const code = `
// camera
camera(
    // aspect_ratio = 1.0,
    image_width = 400,
    image_height = 400,
    samples_per_pixel = 10,
    max_depth = 10,
    vertical_fov = 90.0,
    look_from = [50.0, -50.0, 70.0],
    look_at = [0.0, 0.0, 0.0],
    up = [0.0, 0.0, 1.0],
    defocus_angle = 0.0,
    focus_distance = 10.0,
    background = [0.7, 0.8, 1.0]
);

color([0,125,255]/255)
    scale([1.2,1,1])
    cube([60,20,10],center=true);
`;

export function App() {
  return (
    <Container size="sm" style={{ marginTop: '50px' }}>
      <Title order={1}>Welcome to Mantine!</Title>
      <Text size="lg" mt="md">
        Your React app with Mantine is ready to go.
      </Text>
      <Editor
        height="500px"
        language="javascript"
        theme="vs-dark"
        value={code}
        options={{ minimap: { enabled: false } }}
      />
    </Container>
  );
}


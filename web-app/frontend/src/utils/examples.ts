import type { Project } from '../types';

export enum Example {
    Car = 'Car',
    ThreeSpheres = 'ThreeSpheres',
    RandomSpheres = 'RandomSpheres',
}

export function getExampleProject(example: Example): Project {
    switch (example) {
        case Example.Car:
            return {
                files: [{ filename: 'main.scad', url: '/examples/car/main.scad' }],
            };

        case Example.ThreeSpheres:
            return {
                files: [{ filename: 'main.scad', url: '/examples/three-spheres/main.scad' }],
            };

        case Example.RandomSpheres:
            return {
                files: [{ filename: 'main.scad', url: '/examples/random-spheres/main.scad' }],
            };

        default:
            throw new Error(`unhandled: ${example}`);
    }
}

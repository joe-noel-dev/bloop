import {Entity} from './entity';

export interface Channel extends Entity {
  name: string;
  volume: number;
  mute: boolean;
  solo: boolean;
  colour: string;
}

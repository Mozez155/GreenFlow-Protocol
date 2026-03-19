import { Injectable } from '@nestjs/common';

@Injectable()
export class AppService {
  getHello(): string {
    return 'GreenFlow Protocol backend is ready.';
  }
}

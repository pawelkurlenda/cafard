import http from 'k6/http';
import { sleep, check } from 'k6';

export let options = {
    stages: [
        { duration: '1m', target: 1000 },  // Ramp up to 1000 users within 1 minute
        { duration: '1m', target: 1000 },  // Stay at 1000 users for 1 minute
        { duration: '1m', target: 0 }      // Ramp down to 0 users
    ]
};

export default function () {
    let response = http.get('http://localhost:8080');
    check(response, { 'status is 200': (r) => r.status === 200 });
    sleep(1);
}
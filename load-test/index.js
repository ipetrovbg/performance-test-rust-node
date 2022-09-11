import http from 'k6/http';

export const options = {
  vus: 10,
  duration: '5s',
};

export default function () {
  http.get('https://yjau8mc7qd.execute-api.eu-central-1.amazonaws.com/rust');
  http.get('https://6ucnlpncta.execute-api.eu-central-1.amazonaws.com/node');
}
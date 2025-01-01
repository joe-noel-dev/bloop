const express = require('express');
const mdns = require('mdns');

const app = express();
const port = 3001;

const isSameService = (a, b) =>
  a.name === b.name && a.networkInterface === b.networkInterface;

const removeIPv6 = (service) => {
  service.addresses = service.addresses.filter((address) =>
    address.includes('.')
  );
};

const isIPv4 = (service) =>
  service.addresses.some((address) => address.includes('.'));

const createBrowser = () => {
  const browser = mdns.createBrowser(mdns.tcp('bloop'));
  let services = [];

  browser.on('serviceUp', (service) => {
    if (!isIPv4(service)) {
      console.log('Ignoring non-IPv4 service:', service);
      return;
    }

    removeIPv6(service);

    console.log('Service up:', service);
    services = services.filter((s) => !isSameService(s, service));
    services.push(service);
  });

  browser.on('serviceDown', (service) => {
    console.log('Service down:', service);
    services = services.filter((s) => !isSameService(s, service));
  });

  browser.start();

  return {
    getServices: () => services,
  };
};

const browser = createBrowser();

app.get('/api/discover', (req, res) => {
  const services = browser.getServices();
  res.json({services});
});

app.listen(port, () => {
  console.log(`API server running on http://localhost:${port}`);
});

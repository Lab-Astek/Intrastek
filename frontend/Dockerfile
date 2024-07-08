FROM node:18
WORKDIR /usr/src/frontend
COPY package*.json ./
RUN npm install
COPY . .
CMD ["npm", "run", "dev"]

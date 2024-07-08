FROM node:12-alpine
WORKDIR /usr/src/frontend
COPY package*.json /usr/src/frontend
RUN npm install
COPY . .
ENV PORT=80²
EXPOSE 80
CMD ["npm", "start"]

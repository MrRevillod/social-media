FROM node:lts-alpine as base

WORKDIR /social-net

COPY ./apps/web/package.json ./
RUN npm install
COPY apps/web .

FROM base as dev
EXPOSE 5173
CMD ["npm", "run", "dev"]

FROM base as builder
RUN npm run build

FROM nginx:alpine as prod
COPY --from=prod /app/dist /usr/share/nginx/html
COPY config/nginx.prod.conf /etc/nginx/nginx.conf
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
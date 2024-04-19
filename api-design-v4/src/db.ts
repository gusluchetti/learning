import { PrismaClient } from "@prisma/client";

const prisma = new PrismaClient();
export default prisma;

// TODO: how to deal with the fact that db is down?
// is total api shutdown ok?

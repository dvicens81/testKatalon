<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get Allergy by ID</name>
   <tag></tag>
   <elementGuidId>0a052fd3-4703-4e52-b808-f8cc2b196866</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJlbW1hMTNAeW9wbWFpbC5jb20iLCJncm91cENlbnRlckxpc3QiOlszMDIsMSwwXSwiaXNzIjoiY2FzIiwiaWRHcm91cENlbnRlciI6MSwiaXAiOiIxNzIuMjIuMC4xIiwiZXhwIjoxNTU5Mjk0NTQ4LCJyb2xlRGVzY3JpcHRpb24iOiJEb2N0b3IiLCJzZXNzaW9uSWQiOiJCMzJFQTUxNDIyNDUyQTc3NTdGRjNFNEFFMUE1MUVGMiIsImZ1bmN0aW9uYWxpdGllcyI6WyJNWVBfREFGXzAwMSIsIk1ZUF9IQ0NfMDAyIiwiUk9MRV9BQ0NFU1MiLCJTRUhSX0RPQyJdLCJ1c2VySWQiOjEyNjU1MzgsIndvcmtwbGFjZUlkIjo2ODg5MCwicm9sZSI6NzUwLCJjZW50ZXJJZCI6MzAyfQ.qcqAhTk1ZZnBX87VBAouXvMZ5nRArh28HpNM_H52wus</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://localhost:9098/clevehr-services/api/v1/patients/1265978/allergies/403c4932-d621-4e28-9b17-72d9cb210b31</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>findAllergyByIdUsingGET</name>
   <tag></tag>
   <elementGuidId>4b8a476b-4f9f-422c-803c-9659b383bf2c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-type</name>
      <type>Main</type>
      <value>*/*</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://localhost:9098/clevehr-services/api/v1/patients/${patientId}${/allergies/}${allergyId}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>932a9f82-30ae-4176-9be5-126a880dcf88</id>
      <masked>false</masked>
      <name>allergyId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>072e11d8-8bd0-4da4-92c8-1b0ac179d2cf</id>
      <masked>false</masked>
      <name>patientId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>